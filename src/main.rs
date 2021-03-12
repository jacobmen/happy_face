mod types;
mod server;

use message_io::network::Transport;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

fn main() {
    // let mes = Message::new("jacob", "bob", "hello world");
    // println!("{:?}", mes);
    // let msg_str = bincode::serialize(&mes).unwrap();
    // println!("{:?}", msg_str);
    let tp = Transport::Tcp;
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3044);

    server::run(tp, SocketAddr::V4(addr));
}
