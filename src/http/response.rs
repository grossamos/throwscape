use std::fs;

use crate::configuration::Config;

use super::HttpRequest;

pub struct HttpResponse {
    content: String,
    status_line: String,
}

impl HttpResponse {
    pub fn format(&self) -> String {
        format!(
            "{}\r\n\r\n\r\n{}",
            self.status_line,
            self.content,
        )
    }
}

pub fn respond_to_request(request: &HttpRequest, config: &Config) -> HttpResponse {
    let mut file_path = config.serve_path.clone();

    if request.request_target == "/" {
        file_path.push("index.html");
    } else {
        file_path.push(&request.request_target[1..]);
    }

    let file_path = file_path.as_path();

    let content;
    let status_line;
    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            content = file_content;
            status_line = String::from("HTTP/1.1 200 OK");
        },
        Err(_) => {
            content = String::from("FILE NOT FOUND");
            status_line = String::from("HTTP/1.1 404 NOT FOUND");
        }
    }

    HttpResponse { 
        content,
        status_line,
    }
}
