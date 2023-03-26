use libp2p::gossipsub::IdentTopic;
use once_cell::sync::Lazy;

pub const TotalWeights: u64 = 1800000;

pub const STORAGE_PATH: &str = "./node.json";

pub const ID_PREFIX: [u8; 6] = [0, 36, 8, 1, 18, 32];

pub static CHAIN_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("chain"));

pub static BLOCK_TOPIC: Lazy<IdentTopic> = Lazy::new(|| IdentTopic::new("blocks"));