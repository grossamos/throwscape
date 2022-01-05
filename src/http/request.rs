use std::{net::TcpStream, io::Read};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    _content: Option<String>,
}

impl HttpRequest {
    pub fn new(stream: &mut TcpStream) -> Result<HttpRequest, String> {
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = vec![0 as u8; BUFFER_SIZE];
        let mut header_string = String::new();

        loop {
            let bytes_read = match stream.read(&mut buffer) {
                Ok(num) => num,
                Err(_) => return Err(
                    String::from("An error occured while reading the stream")
                ),
            };

            header_string.push_str(&String::from_utf8_lossy(&buffer));

            if bytes_read < BUFFER_SIZE {
                break;
            }
        }

        let mut header_lines = header_string.lines();
        let status_line = match header_lines.next() {
            Some(status_line) => status_line,
            None => return Err(String::from("Invalid HttpRequest")),
        };


        let (method, path) = Self::parse_statusline(status_line)?;

        Ok(HttpRequest {
            method,
            path,
            _content: None,
        })
        
    }
    
    fn parse_statusline(status_line: &str) -> Result<(HttpMethod, String), String> {
        let mut words = status_line.split_whitespace();
        let method = match words.next() {
            Some(method_string) => match method_string {
                "GET" => HttpMethod::GET,
                _ => return Err(String::from("Invalid HttpMethod")),
            }
            None => return Err(String::from("Missing HttpMethod")),
        };

        let path = match words.next() {
            Some(path) => path,
            None => return Err(String::from("Missing path")),
        };

        if !path.starts_with("/") {
            return Err(String::from("Invalid Path"));
        }

        Ok((method, String::from(path)))
    }

}




