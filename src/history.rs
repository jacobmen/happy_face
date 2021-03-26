extern crate sled;

use sled::{Result, IVec};

use super::types::Message;

// history_db file will have vectors that each represent a conversation (list of messages) 
// with keys representing the unique chat groups/paiers
pub fn insert_message(key: &str, msg: Message) -> Result<()> {
  let db = sled::open("history_db")?;

  if !db.contains_key(key).unwrap() {
    let mut vec: Vec<Message> = Vec::new();
    vec.push(msg);
    let vec_ser = bincode::serialize(&vec).unwrap();
    db.insert(key, vec_ser)?;
  } else {
    let value = db.get(key).unwrap().unwrap();
    let mut history_vector: Vec<Message> = bincode::deserialize(&value).unwrap();
    history_vector.push(msg);
    let vec_ser = bincode::serialize(&history_vector).unwrap();
    db.insert(key, vec_ser)?;
  }

  Ok(())
}

pub fn print_history(key: &str) -> Result<()> {
  let db = sled::open("history_db")?;
  
  let value = db.get(key).unwrap().unwrap();
  let history_vector: Vec<Message> = bincode::deserialize(&value).unwrap();

  for x in 0..history_vector.len() {
    println!("{:?}", history_vector[x]);
  }
  
  Ok(())
}

pub fn clear_history() -> Result<()> {
  let db = sled::open("history_db")?;
  let result = db.clear();

  Ok(())
}