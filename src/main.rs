use std::{net::{TcpListener, SocketAddr}, process, env, sync::Arc};
use throwscape::configuration::Config;

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
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    if let Ok(listener) = TcpListener::bind(addr) {
        throwscape::run(Arc::new(config), listener);
    } else {
        eprintln!("Could not bind to port");
        process::exit(1);
    }
}
