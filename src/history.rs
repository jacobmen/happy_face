extern crate sled;

use sled::{Result, IVec};

use super::types::Message;

pub fn create_history() -> Result<()> {
  let db = sled::open("history_db")?;
  let key = "JaneJohn";
  let mut vec = Vec::new();

  let message = Message::new("jacob", "bob", "hello world");
  vec.push(message);

  let message_data = bincode::serialize(&vec).unwrap();
  dbg!(db.insert(key, message_data)?);
  let value = db.get(key).unwrap().unwrap();
  let output: Vec<Message> = bincode::deserialize(&value).unwrap();
  println!("{:?}", output[0]);
  Ok(())
}

// history_db file will have vectors that each represent a conversation (list of messages) with keys representing the unique
pub fn insert_message(key: &str, msg: Message) {
  let db = sled::open("history_db");
  let msg_data = bincode::serialize(&msg).unwrap();
  
}