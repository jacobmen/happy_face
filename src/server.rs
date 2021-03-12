use std::collections::HashMap;
use std::net::SocketAddr;
use message_io::network::{Network, NetEvent, Endpoint, Transport};

use super::types::Message;

pub fn run(transport: Transport, addr: SocketAddr) {
    let (mut network, mut event_queue) = Network::split();

    let mut clients: HashMap<String, String> = HashMap::new();

    match network.listen(transport, addr) {
        Ok((_resource_id, real_addr)) => {
            println!("Server running at {} by {}", real_addr, transport);
        },
        Err(_) => {
            println!("Can't listen at {} by {}", addr, transport);
        }
    }

    loop {
        match event_queue.receive() {
            NetEvent::Message(endpoint, input_data) => {
                let message: Message = bincode::deserialize(&input_data)
                    .unwrap_or_else(|err| {
                        println!("{}", err);
                        Message::new("", "", "")
                    });
                println!("{:?}", message);
            },
            NetEvent::Connected(endpoint) => {
                // TODO: Add client to map
                println!("Client at {} connected", endpoint.addr());
            },
            NetEvent::Disconnected(endpoint) => {
                // TODO: Remove client
                println!("Client at {} disconnected", endpoint.addr());
            },
        }
    }
}
