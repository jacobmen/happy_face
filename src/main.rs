mod types;
mod server;

use types::Message;

fn main() {
    let mes = Message::new("jacob", "bob", "hello world");
    println!("{:?}", mes);
    let msg_str = bincode::serialize(&mes).unwrap();
    println!("{:?}", msg_str);
}
