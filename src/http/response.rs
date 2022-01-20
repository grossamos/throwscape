use std::fs::File;
use std::io::{Write, Error, BufReader, self};
use std::path::{Path, PathBuf};
use std::net::TcpStream;

use crate::configuration::Config;

use super::request::{HttpRequestTarget, HttpConnectionMetaData, HttpHeader};
use super::{HttpRequest, HttpMethod};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct HttpResponse {
    status: HttpStatus,
    meta_data: HttpConnectionMetaData,
    headers: Vec<HttpHeader>,
    content: HttpMessageContent,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpStatus {
    Okay,
    BadRequest,
    MethodNotAllowed,
    FileNotFound,
    InternalServerError,
    //NotImplemented,
}

impl HttpStatus {
    pub fn as_code(&self) -> i32 {
        match &self {
            HttpStatus::Okay => 200,
            HttpStatus::BadRequest => 400,
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::FileNotFound => 404,
            HttpStatus::InternalServerError => 500,
            //HttpStatus::NotImplemented => 501,
        }
    }
    pub fn as_reason_statement(&self) -> &str {
        match &self {
            HttpStatus::Okay => "OK",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::FileNotFound => "File Not Found",
            //HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
    pub fn get_reason_statement_len(&self) -> u64 {
        self.as_reason_statement().len() as u64
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum HttpMessageContent {
    FileContent(Box<Path>),
    Empty,
    ErrorResponse,
}

impl HttpResponse {
    pub fn new(request: HttpRequest, config: &Config) -> HttpResponse {
        let path = match &request.request_target {
            HttpRequestTarget::OriginForm{path, ..} | 
            HttpRequestTarget::AbsoluteForm { path: Some(path), .. } => path,
            HttpRequestTarget::AbsoluteForm { path: None, .. } => "/",
            _ => return Self::generate_error_response(HttpStatus::BadRequest, request.meta_data),
        };

        let (content, len) = if request.method == HttpMethod::GET {
            match Self::pre_generate_message_content(path, &config.serve_path, &config.index_file_name) {
                Err(status) => return Self::generate_error_response(status, request.meta_data),
                Ok(message) => message,
            }
        } else if request.method == HttpMethod::HEAD {
            (HttpMessageContent::Empty, 0)
        } else {
            return Self::generate_error_response(HttpStatus::MethodNotAllowed, request.meta_data);
        };

        let headers = Self::generate_response_headers(len);

        HttpResponse { 
            status: HttpStatus::Okay,
            meta_data: request.meta_data,
            headers,
            content,
        }
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<(), Error> {
        stream.write(self.generate_status_line().as_bytes())?;
        for header in self.headers.iter() {
            stream.write(header.to_string().as_bytes())?;
        }
        stream.write("\r\n".as_bytes())?;
        match &self.content {
            HttpMessageContent::Empty => {},
            HttpMessageContent::ErrorResponse => {
                // TODO add special handling for 404 case
                stream.write(self.status.as_reason_statement().as_bytes())?;
            }
            HttpMessageContent::FileContent(path) => {
                let mut reader = BufReader::new(File::open(path)?);
                io::copy(&mut reader, stream)?;
            },
        }
        stream.flush()?;
        Ok(())

    }

    fn generate_error_response(status: HttpStatus, meta_data: HttpConnectionMetaData) -> HttpResponse {
        let headers = Self::generate_response_headers(status.get_reason_statement_len());
        HttpResponse { status, meta_data, headers, content: HttpMessageContent::ErrorResponse }
    }

    fn pre_generate_message_content(path: &str, serve_path: &PathBuf, index_file_name: &str) -> Result<(HttpMessageContent, u64), HttpStatus> {
        let mut file_path = serve_path.clone();

        let path = if path.starts_with("/") {
            &path[1..]
        } else {
            &path
        };

        file_path.push(path);

        if file_path.is_dir() {
            file_path.push(index_file_name);
        }

        // prevent fs leaks by checking for subdirectory
        let file_path = match file_path.canonicalize() {
            Ok(file_path) => file_path,
            Err(_) => return Err(HttpStatus::FileNotFound),
        };
        
        if !file_path.starts_with(serve_path) {
            return Err(HttpStatus::FileNotFound)
        }
        let len = match file_path.metadata() {
            Ok(meta_data) => meta_data.len(),
            Err(_) => return Err(HttpStatus::InternalServerError),
        };
        Ok((HttpMessageContent::FileContent(Box::from(file_path.as_path())), len))
    }

    fn generate_status_line(&self) -> String {
        format!(
            "{} {} {}\r\n",
            self.meta_data.http_version.to_string(),
            self.status.as_code(),
            self.status.as_reason_statement(),
        )
    }

    fn generate_response_headers(content_length: u64) -> Vec<HttpHeader> {
        let mut headers = vec![];
        headers.push(HttpHeader{field_name: String::from("Content-Length"), field_value: content_length.to_string()});
        headers
    }

}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::http::HttpResponse;
    use crate::http::request::{HttpVersion, HttpHeader, HttpConnectionMetaData};
    use crate::http::response::{HttpStatus, HttpMessageContent};

    #[test]
    fn request_translates_to_correct_file_path() {
        let index_file_name = "index.html";
        let serve_path = env::current_dir().unwrap();
        let html_path = "/example/";

        let mut expected = serve_path.to_path_buf();
        expected.push("example");
        expected.push(index_file_name);

        let expected = HttpMessageContent::FileContent(Box::from(expected.as_path()));

        let result = HttpResponse::pre_generate_message_content(html_path, &serve_path, index_file_name).unwrap().0;
        assert_eq!(result, expected);
    }

    #[test]
    fn file_system_leaks_are_caught() {
        let index_file_name = "index.html";
        let serve_path = env::current_dir().unwrap();
        let html_path = "../";

        let expected = Err(HttpStatus::FileNotFound);

        let result = HttpResponse::pre_generate_message_content(html_path, &serve_path, index_file_name);
        assert_eq!(result, expected);
    }

    #[test]
    fn invalid_file_returns_file_not_found() {
        let index_file_name = "invalid_html_file.html.stupid";
        let serve_path = env::current_dir().unwrap();
        let html_path = "/example/";

        let expected = Err(HttpStatus::FileNotFound);

        let result = HttpResponse::pre_generate_message_content(html_path, &serve_path, index_file_name);
        assert_eq!(result, expected);
    }

    #[test]
    fn generates_valid_status_line() {
        let version = HttpVersion{major: 1, minor: 1};
        let status_code = HttpStatus::Okay;

        let http_response = HttpResponse{ status: status_code, meta_data: HttpConnectionMetaData{ http_version: version }, headers: vec![], content: HttpMessageContent::Empty };
        
        let expected = String::from("HTTP/1.1 200 OK\r\n");
        let result = http_response.generate_status_line();
        
        assert_eq!(expected, result);
    }

    #[test]
    fn generates_valid_headers() {
        let result = HttpResponse::generate_response_headers(123);
        assert!(result.len() > 0);
        let mut content_length_header = &HttpHeader{field_name: String::new(), field_value: String::new()}; // invalid
        for header in result.iter() {
            if header.field_name == "Content-Length" {
                content_length_header = header;
            }
        }
        assert_eq!(content_length_header, &HttpHeader{field_name: "Content-Length".to_string(), field_value: "123".to_string()});

    }
}
