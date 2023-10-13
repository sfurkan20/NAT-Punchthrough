use std::net::TcpListener;
use std::collections::HashMap;

use crate::request_logger::RequestLogger;
use crate::request_reader::RequestReader;
use crate::rendezvous_manager::RendezvousManager;

mod config;
mod request_reader;
mod request_logger;
mod rendezvous_manager;
mod host_data;

fn main() {
    let mut group_identifier_to_rendezvous_manager: HashMap<String, RendezvousManager> = HashMap::new();
    
    let address = format!("127.0.0.1:{}", config::LISTEN_PORT.to_string());
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let (group_identifier, host_data) = match RequestReader::parse_host_data(&stream) {
            Ok(result) => result,
            Err(message) => {
                RequestLogger::log_error(&stream, &message);
                continue;
            }
        };

        let rendezvous_manager;
        if !group_identifier_to_rendezvous_manager.contains_key(&group_identifier) {
            group_identifier_to_rendezvous_manager.insert(group_identifier.clone(), RendezvousManager::new());
        }
        rendezvous_manager = group_identifier_to_rendezvous_manager.get_mut(&group_identifier).unwrap();
        rendezvous_manager.acquaint_host(stream, host_data);
    }
}