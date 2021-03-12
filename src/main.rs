mod types;
mod server;
mod client;

use message_io::network::Transport;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use message_io::network::{RemoteAddr};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let tp = Transport::Tcp;
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3044);
    
    if (args[1] == "server") {
        server::run(tp, SocketAddr::V4(addr));
    } else {
        client::run(tp, RemoteAddr::SocketAddr(SocketAddr::V4(addr)));
    }
}
