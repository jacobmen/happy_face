extern crate sled;

use sled::{IVec, Result};

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

pub fn get_history(key: &str) -> String {
  let db = sled::open("history_db");

  let value = db.unwrap().get(key).unwrap().unwrap();
  let history_vector: Vec<Message> = bincode::deserialize(&value).unwrap();
  
  // only send 10 most recent Messages
  let mut start_index = 0;
  if history_vector.len() > 10{
    start_index = history_vector.len() - 10;
  }

  let mut history_string = "".to_string();
  for x in start_index..history_vector.len() {
    history_string.push_str(history_vector[x].sender);
    history_string.push_str(": ");
    history_string.push_str(history_vector[x].content);
    history_string.push_str("\n");
  }

  return history_string;
}

pub fn clear_history() -> Result<()> {
    let db = sled::open("history_db")?;
    let result = db.clear();

    Ok(())
}
