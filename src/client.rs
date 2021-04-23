use super::input::InputCollector;
use super::types::{get_message_type, Message, MessageType};
use super::renderer::{render};

use message_io::events::EventQueue;
use message_io::network::{AdapterEvent, NetEvent, Network, RemoteAddr, Transport};

use std::io::{self, Write};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

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
        // enable_raw_mode().expect("can run in raw mode");
        // TODO: 
        // Have an event for character inputs and whenever character is inputted, handle
            // every character to be printed out in the box using TUI and '\n' will be handled automatically 
            // by inputcollector???
            // handle ^C to be disabling raw mode and then quit out of the current running process

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
        // let res = render();
        let mut receiver = "Receiver".to_string();

        // Send garbage initial value so client is recognized later
        let initial_message = Message::new(sender, "", "");
        self.network
            .send(server_id, &bincode::serialize(&initial_message).unwrap());

        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).expect("Couldn't make terminal");
        // let mut text = "hello";
        let mut vec: Vec<String> = Vec::new();
        loop {
            match self.event_queue.receive() {
                // receive response from server
                Event::Network(net_event) => match net_event {
                    NetEvent::Message(_, mes) => {
                        let message: Message = bincode::deserialize(&mes).unwrap_or_else(|err| {
                            println!("{}", err);
                            Message::new("", "", "")
                        });
                        vec.push(message.content.to_string());
                        println!("");  
                        // history
                        // if message.sender == "Server" {
                        //     let split = message.content.split("\n");
                        //     for s in split {
                        //         println!("{}", s);
                        //     }
                        // } else {
                        //     // test_text = message.content;
                        //     println!("{}: {}", message.sender, message.content);
                        // }
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
                        }
                        _ => {
                            let message = Message::new(sender, &receiver, cleaned_str);
                            // vec.push(message.content.to_string());
                            let serialized_data = bincode::serialize(&message).unwrap();
                            self.network.send(server_id, &serialized_data);
                        }
                    }
                }
            }
            terminal.draw(|f| {
                let size = f.size();
                if vec.len() > 0 {
                    let mut messages = "".to_string();
                    for x in 0..vec.len() {
                        messages.push_str(&vec[x]);
                        messages.push_str("\n");
                    }
                    let copyright = Paragraph::new(messages)
                    .style(Style::default().fg(Color::LightCyan))
                    .alignment(Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::White))
                            .border_type(BorderType::Plain),
                    );
                    
                    f.render_widget(copyright, size);
                } else {
                    let b_box = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain);
                
                    f.render_widget(b_box, size);
                }
            }).expect("Failed to draw");
            // print!("> ");
            // io::stdout().flush().expect("Failed to flush stdout");
        }
    }
}
