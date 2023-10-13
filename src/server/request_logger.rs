use std::net::TcpStream;

pub struct RequestLogger {}

impl RequestLogger {
    pub fn log_error(tcp_stream: &TcpStream, message: &String) {
        println!("[ERROR]");
        println!("Sender IP: {}", tcp_stream.local_addr().unwrap().to_string());
        println!("{}\n", message);
    }
}