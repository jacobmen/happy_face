mod client;
mod history;
mod input;
mod server;
mod types;
mod ui;
mod renderer;

use types::Message;

use message_io::network::RemoteAddr;
use message_io::network::Transport;
use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};


fn main() {
    let args: Vec<String> = env::args().collect();

    let tp = Transport::Tcp;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend); 
    // let message = Message::new("jacob", "bob", "hello world");
    // let message2 = Message::new("jacob", "bob", "hello again");
    // let key = "bobjacob";

    // let result = history::insert_message(&key, message);
    // let result2 = history::insert_message(&key, message2);
    // let result3 = history::print_history(&key);

    if args[1] == "server" {
        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3044);
        server::run(tp, SocketAddr::V4(addr));
    } else {
        let client_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3044);
        client::run(
            tp,
            RemoteAddr::SocketAddr(SocketAddr::V4(client_addr)),
            &args[2],
        );
    }
}
