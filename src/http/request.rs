use std::{net::TcpStream, io::{BufReader, BufRead}};

use crate::configuration::Config;

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
    pub fn new(stream: &mut TcpStream, config: &Config) -> Result<HttpRequest, String> {
        stream.set_read_timeout(Some(config.timeout)).unwrap();

        let mut buffered_reader = BufReader::new(stream);

        // TODO check for security in regards to timeouts
        let mut status_line_buffer = String::new();

        match buffered_reader.read_line(&mut status_line_buffer) {
            Err(_) | Ok(0) => return Err(String::from("Issue reading stream")),
            _ => {},
        }

        let (method, path) = Self::parse_statusline(&status_line_buffer)?;

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




