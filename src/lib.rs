use std::net::TcpListener;

pub fn run(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        print!("Connected");
    }
}
