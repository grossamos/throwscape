use std::net::TcpListener;

use http::{HttpRequest, HttpResponse};
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
                    Err(err) => {
                        eprintln!("ERROR: {:?}", err);
                        return;
                    },
                };

                let response = HttpResponse::new(request, &thread_config);
                match response.send(&mut stream) {
                    Ok(_) => {},
                    Err(err) => eprintln!("ERROR: {}", err),
                }

            }
        ));

    }
}


