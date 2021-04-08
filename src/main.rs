mod client;
mod history;
mod input;
mod server;
mod types;

use clap::{crate_authors, crate_version, App, Arg};
use message_io::network::RemoteAddr;
use message_io::network::Transport;
use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use client::Client;
use server::Server;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let matches = App::new("Happy Face")
        .version(crate_version!())
        .about("Terminal based chat app")
        .author(crate_authors!())
        .subcommand(
            App::new("server")
        )
        .subcommand(
            App::new("client")
                .arg(
                    Arg::with_name("client_name")
                        .help("Name of client")
                        .index(1)
                        .required(true),
                )
        ).get_matches();

    if let Some(_) = matches.subcommand_matches("server") {
        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3044);
        let mut server = Server::new();
        server.run(Transport::Tcp, SocketAddr::V4(addr));
    } else if let Some(ref matches) = matches.subcommand_matches("client") {
        let client_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3044);
        let mut client = Client::new();
        client.run(
            Transport::Tcp,
            RemoteAddr::SocketAddr(SocketAddr::V4(client_addr)),
            matches.value_of("client_name").expect("Couldn't decode client name"),
        );
    } else {
        eprintln!("No legal subcommand found. Run with --help for options");
    }
}
