use std::net::TcpListener;

fn main() {
    // open tcp port
    if let Ok(listener) = TcpListener::bind("localhost:8080") {
        ultrascape::run(listener);
    }
}
