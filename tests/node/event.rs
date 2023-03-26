use crate::network::Message;

#[derive(Debug)]
pub enum Event {
    RoundEnd(u64),
    IncomingMessage(Message),
    OutgoingMessage(String, Message),
}