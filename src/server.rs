use message_io::events::EventQueue;
use message_io::network::{Endpoint, NetEvent, Network, Transport};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::cmp::Ordering;

use super::types::{get_message_type, Message, MessageType};
use super::history::{insert_message};

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
        
        let key: String;
        // key is created by appending string of greater value to the lesser valued one
        if message.sender.cmp(message.receiver) == Ordering::Less {
          key = format!("{}{}", message.sender, message.receiver);
        } else {
          key = format!("{}{}", message.receiver, message.sender);
        }

        match get_message_type(&message.content) {
            // Maybe not necessary??
            MessageType::ChangeReceiver => (),
            MessageType::History => self.send_history_info(&message, key),
            MessageType::SendMessage => self.send_message(&message, input_data, key),
        }
    }

    /// Sends message to receiver specified in message receiver field
    fn send_message(&mut self, message: &Message, input_data: &Vec<u8>, key: String) {
        if let Some(receiver_endpt) = self.clients.get(message.receiver) {
            // add to history db
            // todo: handle result?
            let result = insert_message(&key, *message);
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

    fn send_history_info(&mut self, message: &Message, key: String) {
      // sending history to user who originally sent !history
      if let Some(receiver_endpt) = self.clients.get(message.sender) {
        // todo: put in history.rs function
        let db = sled::open("history_db");

        let value = db.unwrap().get(key).unwrap().unwrap();
        let history_vector: Vec<Message> = bincode::deserialize(&value).unwrap();
        
        // only send 10 most recent Messages
        let mut start_index = 0;
        if history_vector.len() > 10{
          start_index = history_vector.len() - 10;
        }

        let mut history_string = "".to_string();
        for x in start_index..history_vector.len() {
          history_string.push_str(history_vector[x].content);
          history_string.push_str("\n");
        }

        let history_msg = Message::new(
            "Server",
            message.sender,
            &history_string
        );

        self.network.send(
            *receiver_endpt,
            &bincode::serialize(&history_msg).unwrap(),
        );
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
