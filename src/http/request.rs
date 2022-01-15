use std::io::{BufReader, BufRead};
use std::net::TcpStream;
use regex::Regex;

use lazy_static::lazy_static;

use crate::configuration::Config;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpParsingError {
    InvalidSyntax,
    TcpIssue(),
    UnknownMethod,
    UnknownScheme,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpRequestTarget {
    // Note: query as defined in RFC3986 is not just key-value based!
    OriginForm{
        path: String,
        query: Option<String>,
    },
    // only relevant for proxies, however HttpSpec requires a server to also accept them
    AbsoluteForm {
        scheme: HttpScheme,
        authority: String,
        path: Option<String>,
        query: Option<String>,
    },
    // only used for CONNECT requests (currently unsupported)
    AuthorityForm {
        authority: String,
    },
    // only used for OPTIONS request
    AsteriskForm,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum HttpScheme {
    Http,
    Https,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct HttpVersion {
    major: u8,
    minor: u8,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub request_target: HttpRequestTarget,
    pub http_version: HttpVersion,
    _content: Option<String>,
}


impl HttpRequest {
    pub fn new(stream: &mut TcpStream, config: &Config) -> Result<HttpRequest, HttpParsingError> {
        stream.set_read_timeout(Some(config.timeout)).unwrap();

        let mut buffered_reader = BufReader::new(stream);

        // TODO check for security in regards to timeouts
        let mut request_line_buffer = String::new();

        match buffered_reader.read_line(&mut request_line_buffer) {
            Err(_) | Ok(0) => return Err(HttpParsingError::TcpIssue()),
            _ => {},
        }

        let (method, request_target, http_version) = Self::parse_request_line(&request_line_buffer)?;

        Ok(HttpRequest {
            method,
            request_target,
            http_version,
            _content: None,
        })
        
    }
    
    fn parse_request_line(request_line: &str) -> Result<(HttpMethod, HttpRequestTarget, HttpVersion), HttpParsingError> {
        let mut lines = request_line.split(" ");

        let method = match lines.next() {
            Some(method_string) => Self::parse_method(method_string)?,
            None => return Err(HttpParsingError::InvalidSyntax),
        };
        let target = match lines.next() {
            Some(target_path_string) => Self::parse_target_path(target_path_string)?,
            None => return Err(HttpParsingError::InvalidSyntax),
        };
        let version = match lines.next() {
            Some(version_string) => Self::parse_http_version(version_string)?,
            None => return Err(HttpParsingError::InvalidSyntax),
        };

        Ok((method, target, version))
    }

    fn parse_method(method: &str) -> Result<HttpMethod, HttpParsingError> {
        match method {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            _ => Err(HttpParsingError::UnknownMethod),
        }

    }

    fn parse_target_path(target_path: &str) -> Result<HttpRequestTarget, HttpParsingError> {
        lazy_static! {
            static ref ORIGIN_FORM_REGEX: Regex = Regex::new(r"^((?:/|\w)*)(?:\?([^#]*))?$").unwrap();
            static ref ABSOLTE_FORM_REGEX: Regex = Regex::new(r"^(http|https)://((?:\w|\.)+(?::\d+)?)((?:/|\w)*)?(?:\?([^#]*))?$").unwrap();
            static ref AUTHORITY_FORM_REGEX: Regex = Regex::new(r"^((?:\w|\.)+(?::\d+)?)$").unwrap();
        }
        match target_path.chars().next().unwrap() {
            '/' => {
                let captures = match ORIGIN_FORM_REGEX.captures(target_path) {
                    Some(captures) => captures,
                    None => return Err(HttpParsingError::InvalidSyntax),
                };

                let path = String::from(&captures[1]);
                let query = match captures.get(2) {
                    Some(rx_match) => Some(String::from(rx_match.as_str())),
                    None => None
                };

                Ok(HttpRequestTarget::OriginForm{
                    path, 
                    query,
                })
            },
            'h' => {
                let captures = match ABSOLTE_FORM_REGEX.captures(target_path) {
                    Some(captures) => captures,
                    None => return Err(HttpParsingError::InvalidSyntax),
                };
                let scheme = match &captures[1] {
                    "http" => HttpScheme::Http,
                    "https" => HttpScheme::Https,
                    _ => return Err(HttpParsingError::UnknownScheme),
                };
                let authority = String::from(&captures[2]);
                let path = match captures.get(3) {
                    Some(rx_match) => Some(String::from(rx_match.as_str())),
                    None => None
                };
                let query = match captures.get(4) {
                    Some(rx_match) => Some(String::from(rx_match.as_str())),
                    None => None
                };
                Ok(HttpRequestTarget::AbsoluteForm{scheme, path, authority, query})
            },
            '*' => {
                if target_path != "*" {
                    Err(HttpParsingError::InvalidSyntax)
                } else {
                    Ok(HttpRequestTarget::AsteriskForm)
                }
            },
            _ => {
                let captures = match AUTHORITY_FORM_REGEX.captures(target_path) {
                    Some(captures) => captures,
                    None => return Err(HttpParsingError::InvalidSyntax),
                };

                let authority = String::from(&captures[1]);

                Ok(HttpRequestTarget::AuthorityForm{authority})
            },
        }
    }

    fn parse_http_version(http_version: &str) -> Result<HttpVersion, HttpParsingError> {
        lazy_static! {
            static ref HTTP_VERSION_REGEX: Regex = Regex::new(r"^HTTP/(\d)\.(\d)$").unwrap();
        }

        let captures = match HTTP_VERSION_REGEX.captures(http_version) {
            Some(captures) => captures,
            None => return Err(HttpParsingError::InvalidSyntax),
        };
        
        // single digit numbers should allways be parseable as u8
        let major = captures[1].parse::<u8>().unwrap();
        let minor = captures[1].parse::<u8>().unwrap();

        Ok(HttpVersion {major, minor})
    }

}


#[cfg(test)]
mod tests {
    use crate::http::{request::{HttpParsingError, HttpScheme}, HttpRequest};

    use super::{HttpMethod, HttpRequestTarget, HttpVersion};

    #[test]
    fn parses_correct_method() {
        let request_line = "GET * HTTP/1.1";
        let result = HttpRequest::parse_request_line(request_line);
        let expected = Ok((HttpMethod::GET, HttpRequestTarget::AsteriskForm, HttpVersion{major: 1, minor: 1}));
        assert_eq!(result, expected);
    }

    #[test]
    fn gives_error_when_providing_invalid_method() {
        let request_line = "GLOOP * HTTP/1.1";
        let result = HttpRequest::parse_request_line(request_line);
        let expected = Err(HttpParsingError::UnknownMethod);
        assert_eq!(result, expected);
    }

    #[test]
    fn gives_error_when_providing_invalid_request_line() {
        let invalid_request_line = "GET * from a good website";
        let result = HttpRequest::parse_request_line(invalid_request_line);
        let expected = Err(HttpParsingError::InvalidSyntax);
        assert_eq!(result, expected);
    }

    #[test]
    fn parses_origin_path_with_parameters_correctly() {
        let valid_target_path = "/hello/world?wuauaua/sj?s._-";
        let result = HttpRequest::parse_target_path(valid_target_path);
        let expected = Ok(HttpRequestTarget::OriginForm{
            path: String::from("/hello/world"),
            query: Some(String::from("wuauaua/sj?s._-")),
        });
        assert_eq!(result, expected);
    }
    
    #[test]
    fn parses_origin_path_without_parameters_correctly() {
        let valid_target_path = "/hello/world";
        let result = HttpRequest::parse_target_path(valid_target_path);
        let expected = Ok(HttpRequestTarget::OriginForm{
            path: String::from("/hello/world"),
            query: None,
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn parses_absolute_path_with_parameters_correctly() {
        let valid_target_path = "http://example.com:8080/hello/world?wuauaua/sj?s._-";
        let result = HttpRequest::parse_target_path(valid_target_path);
        let expected = Ok(HttpRequestTarget::AbsoluteForm {
            scheme: HttpScheme::Http,
            authority: String::from("example.com:8080"),
            path: Some(String::from("/hello/world")),
            query: Some(String::from("wuauaua/sj?s._-")),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn parses_asterisk_path() {
        let valid_asterisk_path = "*";
        let result = HttpRequest::parse_target_path(valid_asterisk_path);
        let expected = Ok(HttpRequestTarget::AsteriskForm);
        assert_eq!(result, expected);
    }

    #[test]
    fn gives_error_for_invalid_asterisk_path() {
        let valid_asterisk_path = "*bluib";
        let result = HttpRequest::parse_target_path(valid_asterisk_path);
        let expected = Err(HttpParsingError::InvalidSyntax);
        assert_eq!(result, expected);
    }

    #[test]
    fn gives_error_for_invalid_path() {
        let invalid_path = "bluib.99:88.ss";
        let result = HttpRequest::parse_target_path(invalid_path);
        let expected = Err(HttpParsingError::InvalidSyntax);
        assert_eq!(result, expected);
    }

    #[test]
    fn parses_authority_form_correctly() {
        let valid_authority_path = "www.example.com:8080";
        let result = HttpRequest::parse_target_path(valid_authority_path);
        let expected = Ok(HttpRequestTarget::AuthorityForm{authority: String::from(valid_authority_path)});
        assert_eq!(result, expected);
    }

    #[test]
    fn parses_http_version_correctly() {
        let http_version = "HTTP/1.1";
        let result = HttpRequest::parse_http_version(http_version);
        let expected = Ok(HttpVersion{major: 1, minor: 1});
        assert_eq!(result, expected);
    }

    #[test]
    fn gives_correct_error_when_parsing_http_version() {
        let http_version = "Amos/1.1";
        let result = HttpRequest::parse_http_version(http_version);
        let expected = Err(HttpParsingError::InvalidSyntax);
        assert_eq!(result, expected);
    }
}


