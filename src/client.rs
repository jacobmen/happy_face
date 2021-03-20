use super::types::Message;

use message_io::network::{NetEvent, Network, RemoteAddr, Transport};

use std::time::Duration;

enum Event {
    Network(NetEvent),
    Message,
}

// remote address: from ec2 instance
pub fn run(transport: Transport, remote_addr: RemoteAddr) {
    let (mut network, mut events) = Network::split_and_map(|net_event| Event::Network(net_event));

    // connect to server
    let (server_id, local_addr) = match network.connect(transport, remote_addr.clone()) {
        Ok(conn_info) => conn_info,
        Err(_) => {
            return println!(
                "Can not connect to the server by {} to {}",
                transport, remote_addr
            );
        }
    };

    println!(
        "Connected to server by {} at {}",
        transport,
        server_id.addr()
    );
    println!("Client identified by local port: {}", local_addr.port());

    loop {
        match events.receive() {
            // send ping to server
            Event::Message => {
                let message = Message::new("jacob", "bob", "hello world");
                let output_data = bincode::serialize(&message).unwrap();
                network.send(server_id, &output_data);
                events.sender().send(Event::Message);
            }
            // receive response from server
            Event::Network(net_event) => match net_event {
                NetEvent::Message(_, mes) => {
                    let message: Message = bincode::deserialize(&mes).unwrap_or_else(|err| {
                        println!("{}", err);
                        Message::new("", "", "")
                    });
                    println!("{:?}", message);
                }
                NetEvent::Connected(_) => unreachable!(), // Only generated when listen
                NetEvent::Disconnected(_) => return println!("Server is disconnected"),
            },
        }
    }
}
