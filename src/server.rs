use message_io::events::EventQueue;
use message_io::network::{Endpoint, NetEvent, Network, Transport};
use std::collections::HashMap;
use std::net::SocketAddr;

use super::types::{get_message_type, Message, MessageType};

pub struct Server {
    network: Network,
    event_queue: EventQueue<NetEvent>,
    clients: HashMap<String, Endpoint>,
}

impl Server {
    /// Creates a new server with a tied network and event queue.
    /// A hashmap to associate sender names with corresponding
    /// endpoints is also included.
    pub fn new() -> Server {
        let (network, event_queue) = Network::split();
        let clients: HashMap<String, Endpoint> = HashMap::new();

        Server {
            network,
            event_queue,
            clients,
        }
    }

    /// Start server to listen on specified socket with transport method
    pub fn run(&mut self, transport: Transport, addr: SocketAddr) {
        match self.network.listen(transport, addr) {
            Ok((_resource_id, real_addr)) => {
                println!("Server running at {} by {}", real_addr, transport);
            }
            Err(_) => {
                println!("Can't listen at {} by {}", addr, transport);
            }
        }

        loop {
            match self.event_queue.receive() {
                NetEvent::Message(endpoint, input_data) => {
                    self.process_message(&input_data, &endpoint);
                }
                NetEvent::Connected(endpoint) => {
                    // TODO: Add client to map
                    println!("Client at {} connected", endpoint.addr());
                }
                NetEvent::Disconnected(endpoint) => {
                    // TODO: Remove client from hashmap
                    println!("Client at {} disconnected", endpoint.addr());
                }
            }
        }
    }

    /// Processes data received from client to determine proper response
    fn process_message(&mut self, input_data: &Vec<u8>, endpoint: &Endpoint) {
        // TODO: Figure out cleaner way to unwrap;
        let message: Message = bincode::deserialize(&input_data).unwrap_or_else(|err| {
            println!("{}", err);
            Message::new("", "", "")
        });

        // Bad message
        if message.sender.is_empty() {
            return;
        }

        println!("{:?}", message);

        // Insert client if new
        if !self.clients.contains_key(message.sender) {
            self.clients.insert(message.sender.to_string(), *endpoint);
        }

        // If message was initial connection
        if message.receiver.is_empty() {
            return;
        }

        match get_message_type(&message.content) {
            // Maybe not necessary??
            MessageType::ChangeReceiver => (),
            MessageType::History => todo!(),
            MessageType::SendMessage => self.send_message(&message, input_data),
        }
    }

    /// Sends message to receiver specified in message receiver field
    fn send_message(&mut self, message: &Message, input_data: &Vec<u8>) {
        if let Some(receiver_endpt) = self.clients.get(message.receiver) {
            self.network.send(*receiver_endpt, input_data);
        } else {
            let err_msg = Message::new(
                "Server",
                message.sender,
                "No reciever with that name connected",
            );

            self.network.send(
                self.clients[message.sender],
                &bincode::serialize(&err_msg).unwrap(),
            );
        }
    }
}
