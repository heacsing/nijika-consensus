use std::{sync::Mutex, collections::HashMap};

use crate::{primitives::{HashValue, NijikaPBFTMessage}, hash::hash};

pub type NijikaMessagePool<'a> = Mutex<HashMap<HashValue, &'a NijikaPBFTMessage>>;

pub enum MessageType {
    INVITE,
    GET_DATA,
    DATA,
}


pub enum MessageDataType {
    DATA_BLOCK,
    PBFT_MSG,
    NETWORK_DATA,
}

trait MessageData {
    fn get_data_type(&self) -> MessageDataType;
    fn as_bytes(&mut self) -> &[u8];
}

pub struct GetDataContent {
    hash: HashValue,
    source_node: HashValue,
}

struct MessageHeader {
    message_type: MessageType,
    timestamp: i64,
    source_node: HashValue,
    data_type: MessageDataType,
    data_hash: HashValue,
}

impl MessageHeader {
    fn new(message_type: MessageType, data_type: MessageDataType, source_node: HashValue, data_hash: HashValue) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        MessageHeader { message_type, timestamp, source_node, data_type, data_hash }
    }
}

pub struct Message {
    header: MessageHeader,
    content: Box<dyn MessageData>,
}

impl Message {
    fn new(source_node: HashValue ,message_type: MessageType, data_type: MessageDataType, mut content: Box<dyn MessageData>) -> Self {
        let data_hash = hash::new(content.as_bytes());
        let header = MessageHeader::new(message_type, data_type, source_node, data_hash);
        Message { header, content }
    }
}


