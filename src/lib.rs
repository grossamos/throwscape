use std::net::TcpListener;
use std::io::Write;

use http::HttpRequest;
use configuration::Config;

use crate::scheduler::ThreadPool;

pub mod http;
pub mod configuration;
pub mod scheduler;

pub fn run(config: Config, listener: TcpListener) {
    let pool = ThreadPool::new(4, config);

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
           
        };

        pool.handle_job(
            Box::new(move |thread_config: &Config| {
                let request = match HttpRequest::new(&mut stream, &thread_config) {
                    Ok(request) => request,
                    Err(_) => return,
                };

                println!("Method: {:?}, Path: {}", request.method, request.path);

                let response = http::respond_to_request(&request, &thread_config);

                stream.write(response.format().as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        ));

    }
}


