use std::{fs, path::{Path, PathBuf}, net::TcpStream};

use crate::configuration::Config;

use super::{HttpRequest, request::{HttpRequestTarget, HttpConnectionMetaData, HttpVersion}};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct HttpResponse {
    status: HttpStatus,
    meta_data: HttpConnectionMetaData,
    content: HttpMessageContent,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpStatus {
    MethodNotAllowed,
    FileNotFound,
}

impl HttpStatus {
    fn as_code(&self) -> i32 {
        match &self {
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::FileNotFound => 404,
        }
    }
    fn as_reason_statement(&self) -> &str {
        match &self {
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::FileNotFound => "File Not Found",
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum HttpMessageContent {
    FileContent(Box<Path>),
    ErrorResponse,
}

impl HttpResponse {
    pub fn new(request: &HttpRequest, config: &Config) -> HttpResponse {
        let path = match &request.request_target {
            HttpRequestTarget::OriginForm{path, query: _} | 
            HttpRequestTarget::AbsoluteForm { scheme: _, authority: _, path: Some(path), query: _ } => path,
            HttpRequestTarget::AbsoluteForm { scheme: _, authority: _, path: None, query: _ } => "/",
            _ => "",
        };
        let _message = Self::pre_generate_message_content(path, &config.serve_path, &config.index_file_name);

        HttpResponse { 
            status: HttpStatus::FileNotFound,
            meta_data: HttpConnectionMetaData { http_version: HttpVersion { major: 1, minor: 1 }},
            content: HttpMessageContent::ErrorResponse,
        }
    }

    pub fn send(&self, stream: &mut TcpStream) {
    }

    fn generate_error_response(status: HttpStatus, meta_data: HttpConnectionMetaData) -> HttpResponse {
        HttpResponse { status, meta_data, content: HttpMessageContent::ErrorResponse }
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

}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::http::response::{HttpMessageContent, HttpStatus};

    use super::HttpResponse;

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
}
