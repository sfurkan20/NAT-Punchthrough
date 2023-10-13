use std::{io::{BufRead, Read}, net::TcpStream};

use nat_punchthrough::acquaint_response::HEADERS;
use crate::host_data::HostData;
use crate::config::RESPONSE_BUFFER_SIZE;

pub struct RequestReader {}

impl RequestReader {
    pub fn parse_host_data(tcp_stream: &mut TcpStream) -> Result<HostData, String> {
        let mut buffer = [0u8; RESPONSE_BUFFER_SIZE];
        tcp_stream.read(&mut buffer).unwrap();

        let mut line_counter = 0;
        let request_lines: Vec<String> = buffer
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
                ipv4: request_lines[0].clone(),
                port: request_lines[1].parse::<u16>().unwrap()
            };

            return Ok(host);
        }

        return Err("Request payload is invalid.".to_string());
    }
}