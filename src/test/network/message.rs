use std::{sync::Mutex, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::{primitives::{HashValue, NijikaPBFTMessage}, hash::hash};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MessageType {
    Invite,
    GetData,
    Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MessageDataType {
    DataBlock,
    DataBlockHash,
    PBFTMsg,
    PBFTMsgHash,
    NetworkData,
    NetworkDataHash,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDataContent {
    hash: HashValue,
    source_node: HashValue,
}
#[derive(Debug, Serialize, Deserialize)]
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

pub trait MessageData {
    fn as_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;
}

#[derive(Debug, Serialize)]
pub struct Message<C>
where C: MessageData + for<'a> Deserialize<'a>
{
    header: MessageHeader,
    content: C,
}

impl<C: MessageData + for<'a> Deserialize<'a>> Message<C> {
    pub fn new_message(source_node: HashValue ,message_type: MessageType, data_type: MessageDataType, content: C) -> Self {
        let binary_data = content.as_bytes();
        let data_hash = hash::new(&binary_data);
        let header = MessageHeader::new(message_type, data_type, source_node, data_hash);
        Message {
            header,
            content
        }
    }
    pub fn get_data_type(&self) -> MessageDataType {
        self.header.data_type
    }
}