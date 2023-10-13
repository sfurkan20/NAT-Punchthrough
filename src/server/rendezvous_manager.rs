use std::{net::{TcpStream, IpAddr}, io::{BufWriter, Write}};

use nat_punchthrough::acquaint_response;
use crate::host_data::HostData;

pub struct RendezvousManager {
    hosts: Vec<(TcpStream, HostData)>
}

impl RendezvousManager {
    pub fn new() -> Self {
        let hosts = vec![];
        Self { hosts }
    }

    fn send_acquaintance_response(&self, target_stream: &TcpStream, source_host_ip: &IpAddr, source_host_data: &HostData) {
        let content = format!("{}:{}\n{}:{}", acquaint_response::HEADERS[0], source_host_ip,
                                                      acquaint_response::HEADERS[1], source_host_data.port);

        let mut write_buffer = BufWriter::new(target_stream);
        write_buffer.write(content.as_bytes()).unwrap();
        write_buffer.flush().unwrap();
    }

    pub fn acquaint_host(&mut self, new_host_stream: TcpStream, new_host_data: HostData) {
        for (stream, host_data) in &self.hosts {
            self.send_acquaintance_response(&stream, &new_host_stream.peer_addr().unwrap().ip(), &new_host_data);
            self.send_acquaintance_response(&new_host_stream, &stream.peer_addr().unwrap().ip(), host_data);
        }
        self.hosts.push((new_host_stream, new_host_data));
    }
}