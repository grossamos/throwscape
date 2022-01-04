use std::{net::{TcpListener, SocketAddr}, process, env};
use crate::configuration::Config;

mod configuration;

fn main() {
    // retrieve configuration
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(conf) => conf,
        Err(error_msg) => {
            eprintln!("ERROR: {}", error_msg);
            process::exit(1);
        }
    }; 

    // open tcp port
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    if let Ok(listener) = TcpListener::bind(addr) {
        ultrascape::run(listener);
    } else {
        eprintln!("Could not bind to port");
        process::exit(1);
    }
}
