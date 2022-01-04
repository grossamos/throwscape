use std::net::TcpListener;

pub fn run(listener: TcpListener) {
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connected");
    }
}
