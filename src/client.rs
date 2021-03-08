use super::types::{Message};

use message_io::network::{Network, NetEvent, Transport, RemoteAddr};

use std::time::{Duration};

enum Event {
    Network(NetEvent),
    Message,
}

// remote address: from ec2 instance
pub fn run(transport: Transport, remote_addr: RemoteAddr) {
  let (mut events, mut network) = Network::split_and_map(|net_event| Event::Network(net_event));
  
  // connect to server
  let (server_id, local_addr) = match network.connect(transport, remote_addr.clone()) {
    Ok(conn_info) => conn_info,
    Err(_) => {
      return println!("Can not connect to the server by {} to {}", transport, remote_addr);
    }
  };

  println!("Connected to server by {} at {}", transport, server_id.addr());
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
              let msg_str = serde_json::to_string(&mes).unwrap();
              println!("{}", msg_str);
            }
            NetEvent::Connected(_) => unreachable!(), // Only generated when listen
            NetEvent::Disconnected(_) => return println!("Server is disconnected"),
        },
    }
}
}