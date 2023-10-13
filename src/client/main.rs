use std::{net::TcpStream, io::Write, env};

use host_data::HostData;
use nat_punchthrough::acquaint_request;
use request_reader::RequestReader;

mod config;
mod request_reader;
mod host_data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let group_identifier = &args[1];
    let inbound_port = args[2].parse::<u16>().unwrap();

    let mut tcp_stream = TcpStream::connect(config::RELAY_SERVER_IPV4_ENDPOINT).unwrap();
    let content = format!("{}:{}\n{}:{}", acquaint_request::HEADERS[0], group_identifier,
                                                  acquaint_request::HEADERS[1], inbound_port);
    tcp_stream.write(content.as_bytes()).unwrap();

    let mut hosts: Vec<HostData> = vec![];
    loop {
        let host_data = RequestReader::parse_host_data(&mut tcp_stream).unwrap();
        println!("New peer has connected! IP: {} Port: {}", host_data.ipv4, host_data.port);
        hosts.push(host_data);
    }
}