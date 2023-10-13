use std::{io::{BufReader, BufRead}, net::TcpStream};

use nat_punchthrough::acquaint_request::HEADERS;
use crate::host_data::HostData;

pub struct RequestReader {}

impl RequestReader {
    pub fn parse_host_data(tcp_stream: &TcpStream) -> Result<(String, HostData), String> {
        let mut line_counter = 0;
        let request_lines: Vec<String> = BufReader::new(tcp_stream)
                    .lines()
                    .map(|line| line.unwrap())
                    .take_while(
                        |line|
                        {
                            if line.is_empty() {
                                return false;
                            }

                            let header_part = line.split(':').nth(0).unwrap_or("");
                            let is_valid = header_part == HEADERS[line_counter];
                            line_counter += 1;
                            return is_valid;
                        })
                    .collect();

        if line_counter == HEADERS.len() {
            let host = HostData
            {
                port: request_lines[1].parse::<u16>().unwrap()
            };

            println!("Port: {}", host.port);

            return Ok((request_lines[0].clone(), host));
        }

        return Err("Request payload is invalid.".to_string());
    }
}