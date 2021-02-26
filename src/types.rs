use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<'a> {
    pub sender: &'a str,
    pub reciever: &'a str,
    pub content: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(sender: &'a str, reciever: &'a str, content: &'a str) -> Message<'a> {
        Message {
            sender,
            reciever,
            content,
        }
    }
}
