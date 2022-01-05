use std::{net::TcpListener, io::Write};

use http::HttpRequest;
use configuration::Config;

pub mod http;
pub mod configuration;

pub fn run(config: Config, listener: TcpListener) {
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
           
        };
        
        let request = match HttpRequest::new(&mut stream) {
            Ok(request) => request,
            Err(_) => continue,
        };

        println!("Method: {:?}, Path: {}", request.method, request.path);

        let response = http::respond_to_request(request, &config);

        stream.write(response.format().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}


