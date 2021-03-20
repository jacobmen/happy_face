use super::input::InputCollector;
use super::types::Message;

use message_io::events::EventQueue;
use message_io::network::{AdapterEvent, NetEvent, Network, RemoteAddr, Transport};

enum Event {
    Network(NetEvent),
    Input(String),
    Message,
}

// remote address: from ec2 instance
pub fn run(transport: Transport, remote_addr: RemoteAddr) {
    let mut event_queue = EventQueue::new();

    let network_queue = event_queue.sender().clone();
    let mut network = Network::new(move |net_event| match net_event {
        AdapterEvent::Removed(_) => {
            network_queue.send_with_priority(Event::Network(NetEvent::from(net_event)))
        }
        _ => network_queue.send(Event::Network(NetEvent::from(net_event))),
    });

    let input_queue = event_queue.sender().clone();
    let input = InputCollector::new(move |input_event| match input_event {
        Ok(event) => input_queue.send(Event::Input(event)),
        Err(event) => input_queue.send(Event::Input("Error".to_string())),
    });

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
        match event_queue.receive() {
            // send ping to server
            Event::Message => {
                let message = Message::new("jacob", "bob", "hello world");
                let output_data = bincode::serialize(&message).unwrap();
                network.send(server_id, &output_data);
                event_queue.sender().send(Event::Message);
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
            // Recieve input from stdin
            Event::Input(user_input) => {
                let cleaned_str = user_input.trim();

                if cleaned_str.len() >= 400 {
                    println!("Bad input: Input too long");
                    continue;
                }

                let message = Message::new("Sender", "Reciever", cleaned_str);
                let output_data = bincode::serialize(&message).unwrap();
                network.send(server_id, &output_data);
                // event_queue.sender().send(Event::Message);
            }
        }
    }
}
