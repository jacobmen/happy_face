use super::input::InputCollector;
use super::types::{get_message_type, Message, MessageType};

use message_io::events::EventQueue;
use message_io::network::{AdapterEvent, NetEvent, Network, RemoteAddr, Transport};

enum Event {
    Network(NetEvent),
    Input(String),
}

pub struct Client {
    network: Network,
    event_queue: EventQueue<Event>,
}

impl Client {
    /// Creates new client with network and general event queue.
    pub fn new() -> Client {
        let mut event_queue = EventQueue::new();

        let network_queue = event_queue.sender().clone();
        let network = Network::new(move |net_event| match net_event {
            AdapterEvent::Removed(_) => {
                network_queue.send_with_priority(Event::Network(NetEvent::from(net_event)))
            }
            _ => network_queue.send(Event::Network(NetEvent::from(net_event))),
        });

        Client {
            network,
            event_queue,
        }
    }

    /// Starts the client, connecting it to the address associated with remote_addr.
    /// Additionally, starts new thread to read input from stdin to use as message content.
    pub fn run(&mut self, transport: Transport, remote_addr: RemoteAddr, sender: &str) {
        let input_queue = self.event_queue.sender().clone();
        let input = InputCollector::new(move |input_event| match input_event {
            Ok(event) => input_queue.send(Event::Input(event)),
            Err(event) => input_queue.send(Event::Input("Error".to_string())),
        });

        let (server_id, local_addr) = match self.network.connect(transport, remote_addr.clone()) {
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

        let mut receiver = "Receiver".to_string();

        // Send garbage initial value so client is recognized later
        let initial_message = Message::new(sender, "", "");
        self.network
            .send(server_id, &bincode::serialize(&initial_message).unwrap());

        loop {
            match self.event_queue.receive() {
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

                    if cleaned_str.len() >= 400 || cleaned_str.is_empty() {
                        println!("Bad input: Input too long / Empty");
                        continue;
                    }

                    match get_message_type(&cleaned_str) {
                        MessageType::ChangeReceiver => {
                            receiver = cleaned_str[1..].to_string();
                            println!("Changed receiver to: {}", receiver);
                            continue;
                        }
                        _ => (),
                    }

                    let message = Message::new(sender, &receiver, cleaned_str);
                    let serialized_data = bincode::serialize(&message).unwrap();
                    self.network.send(server_id, &serialized_data);
                }
            }
        }
    }
}
