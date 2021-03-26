mod client;
mod input;
mod server;
mod types;
mod history;

use types::Message;

use message_io::network::RemoteAddr;
use message_io::network::Transport;
use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};

fn main() {
    let args: Vec<String> = env::args().collect();

    let tp = Transport::Tcp;

    let message = Message::new("jacob", "bob", "hello world");
    let message2 = Message::new("jacob", "bob", "hello again");
    let key = "bobjacob";

    let result = history::insert_message(&key, message);
    let result2 = history::insert_message(&key, message2);
    let result3 = history::print_history(&key);

    if args[1] == "server" {
        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3044);
        server::run(tp, SocketAddr::V4(addr));
    } else {
        let client_addr = SocketAddrV4::new(
            Ipv4Addr::new(0, 0, 0, 0),
            3044,
            // get_free_port().expect("No open ports"),
        );
        client::run(tp, RemoteAddr::SocketAddr(SocketAddr::V4(client_addr)));
    }
}

fn get_free_port() -> Option<u16> {
    (1025..65535).find(|port| match TcpListener::bind(("127.0.0.1", *port)) {
        Ok(_) => true,
        Err(_) => false,
    })
}
