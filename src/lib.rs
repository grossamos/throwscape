use std::{net::TcpListener, sync::Arc};

use configuration::Config;
use http::{HttpRequest, HttpResponse};

use crate::scheduler::ThreadPool;

pub mod configuration;
pub mod http;
pub mod scheduler;

pub fn run(config: Arc<Config>, listener: TcpListener) {
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
        };

        let config = Arc::clone(&config);

        pool.handle_job(Box::new(move || {
            let request = match HttpRequest::new(&mut stream, &config) {
                Ok(request) => request,
                Err(err) => {
                    if config.is_in_debug_mode {
                        eprintln!("Request Error: {:?}", err);
                    }
                    return;
                }
            };

            let response = HttpResponse::new(request, &config);
            match response.send(&mut stream) {
                Ok(_) => {}
                Err(err) => {
                    if config.is_in_debug_mode {
                        eprintln!("Response Error: {}", err);
                    }
                }
            }
        }));
    }
}
