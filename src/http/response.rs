use std::path::{Path, PathBuf};
use std::net::TcpStream;

use crate::configuration::Config;

use super::request::{HttpRequestTarget, HttpConnectionMetaData, HttpHeader, HttpVersion};
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
    //NotImplemented,
}

impl HttpStatus {
    pub fn as_code(&self) -> i32 {
        match &self {
            HttpStatus::Okay => 200,
            HttpStatus::BadRequest => 400,
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::FileNotFound => 404,
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
        }
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

        let content = if request.method == HttpMethod::GET {
            match Self::pre_generate_message_content(path, &config.serve_path, &config.index_file_name) {
                Err(status) => return Self::generate_error_response(status, request.meta_data),
                Ok(message) => message,
            }
        } else if request.method == HttpMethod::HEAD {
            HttpMessageContent::Empty
        } else {
            return Self::generate_error_response(HttpStatus::MethodNotAllowed, request.meta_data);
        };

        // TODO imlement propper length fetching
        let headers = Self::generate_response_headers(123);

        HttpResponse { 
            status: HttpStatus::Okay,
            meta_data: request.meta_data,
            headers,
            content,
        }
    }

    pub fn send(&self, stream: &mut TcpStream) {
        // TODO actually implement send

    }

    fn generate_error_response(status: HttpStatus, meta_data: HttpConnectionMetaData) -> HttpResponse {
        let headers = Self::generate_response_headers(123);
        HttpResponse { status, meta_data, headers, content: HttpMessageContent::ErrorResponse }
    }

    fn pre_generate_message_content(path: &str, serve_path: &PathBuf, index_file_name: &str) -> Result<HttpMessageContent, HttpStatus> {
        let mut file_path = serve_path.clone();

        let path = if path.starts_with("/") {
            &path[1..]
        } else {
            &path
        };

        file_path.push(path);

        if path.ends_with("/") {
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
        Ok(HttpMessageContent::FileContent(Box::from(file_path.as_path())))
    }

    fn generate_status_line(version: HttpVersion, status_code: HttpStatus) -> String {
        format!(
            "{} {} {}\r\n",
            version.to_string(),
            status_code.as_code(),
            status_code.as_reason_statement(),
        )
    }

    fn generate_response_headers(content_length: u32) -> Vec<HttpHeader> {
        let mut headers = vec![];
        headers.push(HttpHeader{field_name: String::from("Content-Length"), field_value: content_length.to_string()});
        headers
    }

}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::http::HttpResponse;
    use crate::http::request::{HttpVersion, HttpHeader};
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

        let result = HttpResponse::pre_generate_message_content(html_path, &serve_path, index_file_name).unwrap();
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
        
        let expected = String::from("HTTP/1.1 200 OK\r\n");
        let result = HttpResponse::generate_status_line(version, status_code);
        
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
