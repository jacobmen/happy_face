use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Message<'a> {
    pub sender: &'a str,
    pub receiver: &'a str,
    pub content: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(sender: &'a str, reciever: &'a str, content: &'a str) -> Message<'a> {
        Message {
            sender,
            receiver: reciever,
            content,
        }
    }
}

pub enum MessageType {
    ChangeReceiver,
    History,
    SendMessage,
}

pub fn get_message_type(message: &str) -> MessageType {
    if message == "!history" {
        MessageType::History
    } else if message
        .chars()
        .nth(0)
        .expect("Message content empty")
        == '!'
    {
        MessageType::ChangeReceiver
    } else {
        MessageType::SendMessage
    }
}

