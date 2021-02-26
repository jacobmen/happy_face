mod types;

use types::Message;

fn main() {
    let mes = Message::new("jacob", "bob", "hello world");
    println!("{:?}", mes);
    let msg_str = serde_json::to_string(&mes).unwrap();
    println!("{}", msg_str);
}
