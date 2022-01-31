use std::fs::File;
use std::io::{copy, BufReader, Error, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use crate::configuration::Config;

use super::request::{HttpConnectionMetaData, HttpHeader, HttpRequestTarget};
use super::{HttpMethod, HttpRequest};

#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    status: HttpStatus,
    meta_data: HttpConnectionMetaData,
    headers: Vec<HttpHeader>,
    content: HttpMessageContent,
}

#[derive(Debug, PartialEq)]
pub enum HttpStatus {
    Okay,
    BadRequest,
    MethodNotAllowed,
    FileNotFound,
    _InternalServerError,
    NotImplemented,
}

impl HttpStatus {
    pub fn as_code(&self) -> i32 {
        match &self {
            HttpStatus::Okay => 200,
            HttpStatus::BadRequest => 400,
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::FileNotFound => 404,
            HttpStatus::_InternalServerError => 500,
            HttpStatus::NotImplemented => 501,
        }
    }
    pub fn as_reason_statement(&self) -> &str {
        match &self {
            HttpStatus::Okay => "OK",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::FileNotFound => "File Not Found",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::_InternalServerError => "Internal Server Error",
        }
    }
    pub fn is_error(&self) -> bool {
        self.as_code() > 400
    }
    pub fn get_reason_statement_len(&self) -> u64 {
        self.as_reason_statement().len() as u64
    }
}

type HttpMessageContent = Option<Box<Path>>;

impl HttpResponse {
    pub fn new(request: HttpRequest, config: &Config) -> HttpResponse {
        let path = match &request.request_target {
            HttpRequestTarget::OriginForm { path, .. }
            | HttpRequestTarget::AbsoluteForm {
                path: Some(path), ..
            } => path,
            HttpRequestTarget::AbsoluteForm { path: None, .. } => "/",
            _ => {
                return Self::generate_error_response(
                    HttpStatus::BadRequest,
                    request.meta_data,
                    config,
                )
            }
        };

        let (content, len) = if request.method == HttpMethod::GET {
            match Self::pre_generate_message_content(
                path,
                &config.serve_path,
                &config.index_file_name,
            ) {
                Err(status) => {
                    return Self::generate_error_response(status, request.meta_data, config)
                }
                Ok(message) => message,
            }
        } else if request.method == HttpMethod::HEAD {
            (None, 0)
        } else if request.method == HttpMethod::UnknownMethod {
            return Self::generate_error_response(
                HttpStatus::NotImplemented,
                request.meta_data,
                config,
            );
        } else {
            return Self::generate_error_response(
                HttpStatus::MethodNotAllowed,
                request.meta_data,
                config,
            );
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
        const DELIMITER: &str = "\r\n";

        stream.write(self.generate_status_line().as_bytes())?;
        stream.write(DELIMITER.as_bytes())?;

        for header in self.headers.iter() {
            stream.write(header.to_string().as_bytes())?;
        }

        stream.write(DELIMITER.as_bytes())?;

        let mut content_missing = true;

        if let Some(content_path) = &self.content {
            match File::open(content_path) {
                Ok(path) => {
                    copy(&mut BufReader::new(path), stream)?;
                    content_missing = false;
                }
                Err(_) => {}
            }
            let file = File::open(content_path)?;
            copy(&mut BufReader::new(file), stream)?;
        }
        println!("{} {}", content_missing, self.status.is_error());
        if self.status.is_error() && content_missing {
            stream.write(self.status.as_reason_statement().as_bytes())?;
        }

        stream.flush()?;
        Ok(())
    }

    fn generate_error_response(
        status: HttpStatus,
        meta_data: HttpConnectionMetaData,
        config: &Config,
    ) -> HttpResponse {
        let len;
        let content;
        if status == HttpStatus::FileNotFound {
            match Self::get_file_length(&config.file_not_found_path) {
                Some(file_len) => {
                    len = file_len;
                    // TODO: remove clone as soon as config is migrated
                    content = Some(Box::from(config.file_not_found_path.clone()));
                }
                None => {
                    // TODO remove duplicate code (see else block below)
                    len = status.get_reason_statement_len();
                    content = None;
                }
            }
        } else {
            len = status.get_reason_statement_len();
            content = None;
        }
        let headers = Self::generate_response_headers(len);
        HttpResponse {
            status,
            meta_data,
            headers,
            content,
        }
    }

    fn pre_generate_message_content(
        path: &str,
        serve_path: &PathBuf,
        index_file_name: &str,
    ) -> Result<(HttpMessageContent, u64), HttpStatus> {
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
            return Err(HttpStatus::FileNotFound);
        }
        let len = match Self::get_file_length(&file_path) {
            Some(len) => len,
            None => return Err(HttpStatus::FileNotFound),
        };
        Ok((Some(Box::from(file_path.as_path())), len))
    }

    // Can also be used as a file existance check
    fn get_file_length(path: &Path) -> Option<u64> {
        match path.metadata() {
            Ok(meta_data) => Some(meta_data.len()),
            Err(_) => None,
        }
    }

    fn generate_status_line(&self) -> String {
        format!(
            "{} {} {}",
            self.meta_data.http_version.to_string(),
            self.status.as_code(),
            self.status.as_reason_statement(),
        )
    }

    fn generate_response_headers(content_length: u64) -> Vec<HttpHeader> {
        let mut headers = vec![];
        headers.push(HttpHeader {
            field_name: String::from("Content-Length"),
            field_value: content_length.to_string(),
        });
        headers
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::http::request::{HttpConnectionMetaData, HttpHeader, HttpVersion};
    use crate::http::response::HttpStatus;
    use crate::http::HttpResponse;

    #[test]
    fn request_translates_to_correct_file_path() {
        let index_file_name = "index.html";
        let serve_path = env::current_dir().unwrap();
        let html_path = "/example/";

        let mut expected = serve_path.to_path_buf();
        expected.push("example");
        expected.push(index_file_name);

        let expected = Some(Box::from(expected.as_path()));

        let result =
            HttpResponse::pre_generate_message_content(html_path, &serve_path, index_file_name)
                .unwrap()
                .0;
        assert_eq!(result, expected);
    }

    #[test]
    fn file_system_leaks_are_caught() {
        let index_file_name = "index.html";
        let serve_path = env::current_dir().unwrap();
        let html_path = "../";

        let expected = Err(HttpStatus::FileNotFound);

        let result =
            HttpResponse::pre_generate_message_content(html_path, &serve_path, index_file_name);
        assert_eq!(result, expected);
    }

    #[test]
    fn invalid_file_returns_file_not_found() {
        let index_file_name = "invalid_html_file.html.stupid";
        let serve_path = env::current_dir().unwrap();
        let html_path = "/example/";

        let expected = Err(HttpStatus::FileNotFound);

        let result =
            HttpResponse::pre_generate_message_content(html_path, &serve_path, index_file_name);
        assert_eq!(result, expected);
    }

    #[test]
    fn generates_valid_status_line() {
        let version = HttpVersion { major: 1, minor: 1 };
        let status_code = HttpStatus::Okay;

        let http_response = HttpResponse {
            status: status_code,
            meta_data: HttpConnectionMetaData {
                http_version: version,
            },
            headers: vec![],
            content: None,
        };

        let expected = String::from("HTTP/1.1 200 OK");
        let result = http_response.generate_status_line();

        assert_eq!(expected, result);
    }

    #[test]
    fn generates_valid_headers() {
        let result = HttpResponse::generate_response_headers(123);
        assert!(result.len() > 0);
        let mut content_length_header = &HttpHeader {
            field_name: String::new(),
            field_value: String::new(),
        }; // invalid
        for header in result.iter() {
            if header.field_name == "Content-Length" {
                content_length_header = header;
            }
        }
        assert_eq!(
            content_length_header,
            &HttpHeader {
                field_name: "Content-Length".to_string(),
                field_value: "123".to_string()
            }
        );
    }

    #[test]
    fn http_error_is_error() {
        assert_eq!(HttpStatus::FileNotFound.is_error(), true);
        assert_eq!(HttpStatus::MethodNotAllowed.is_error(), true);
        assert_eq!(HttpStatus::BadRequest.is_error(), true);
    }

    #[test]
    fn http_ok_is_no_error() {
        assert_eq!(HttpStatus::Okay.is_error(), false);
    }
}
