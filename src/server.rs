use message_io::network::{Endpoint, NetEvent, Network, Transport};
use std::collections::HashMap;
use std::net::SocketAddr;

use super::types::Message;

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
                send_message(&mut network, &mut clients, &input_data, &endpoint);
            }
            NetEvent::Connected(endpoint) => {
                // TODO: Add client to map
                println!("Client at {} connected", endpoint.addr());
            }
            NetEvent::Disconnected(endpoint) => {
                // TODO: Remove client
                println!("Client at {} disconnected", endpoint.addr());
            }
        }
    }
}

// TODO: Return a result for loop to use
fn send_message(
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

    if message.sender == "" {
        return;
    }

    println!("{:?}", message);

    if !clients.contains_key(message.sender) {
        clients.insert(message.sender.to_string(), *endpoint);
    }

    if let Some(reciever_endpt) = clients.get(message.reciever) {
        network.send(*reciever_endpt, input_data);
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
