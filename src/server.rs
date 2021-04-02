use message_io::network::{Endpoint, NetEvent, Network, Transport};
use std::collections::HashMap;
use std::net::SocketAddr;

use super::types::{Message, MessageType, get_message_type};

pub fn run(transport: Transport, addr: SocketAddr) {
    let (mut network, mut event_queue) = Network::split();

    let mut clients: HashMap<String, Endpoint> = HashMap::new();

    match network.listen(transport, addr) {
        Ok((_resource_id, real_addr)) => {
            println!("Server running at {} by {}", real_addr, transport);
        }
        Err(_) => {
            println!("Can't listen at {} by {}", addr, transport);
        }
    }

    loop {
        match event_queue.receive() {
            NetEvent::Message(endpoint, input_data) => {
                process_message(&mut network, &mut clients, &input_data, &endpoint);
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

// TODO: Return a result for loop to use
fn process_message(
    network: &mut Network,
    clients: &mut HashMap<String, Endpoint>,
    input_data: &Vec<u8>,
    endpoint: &Endpoint,
) {
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
    if !clients.contains_key(message.sender) {
        clients.insert(message.sender.to_string(), *endpoint);
    }

    // If message was initial connection
    if message.receiver.is_empty() {
        return;
    }

    match get_message_type(&message.content) {
        // Maybe not necessary??
        MessageType::ChangeReceiver => (),
        MessageType::History => (),
        MessageType::SendMessage => send_message(&message, network, clients, input_data),
    }
}

fn send_message(
    message: &Message,
    network: &mut Network,
    clients: &mut HashMap<String, Endpoint>,
    input_data: &Vec<u8>,
) {
    if let Some(receiver_endpt) = clients.get(message.receiver) {
        network.send(*receiver_endpt, input_data);
    } else {
        let err_msg = Message::new(
            "Server",
            message.sender,
            "No reciever with that name connected",
        );

        network.send(
            clients[message.sender],
            &bincode::serialize(&err_msg).unwrap(),
        );
    }
}
