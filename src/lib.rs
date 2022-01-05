use std::{net::TcpListener, io::Write};

use http::{HttpRequest, HttpMethod};

pub mod http;
pub mod configuration;

pub fn run(listener: TcpListener) {
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(str) => str,
            Err(_) => {
                eprintln!("fuck");
                continue;
            }
        };
        
        let request = match HttpRequest::new(&mut stream) {
            Ok(request) => request,
            Err(_) => continue,
        };

        let response;

        if HttpMethod::GET == request.method && request.path == "/" {
            let content = "<h1>Hello World!</h1>";
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );
        } else {
            response = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n\r\n");
        }
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        println!("Method: {:?}, Path: {}", request.method, request.path)
    }
}


