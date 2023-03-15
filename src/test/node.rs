use std::{collections::HashMap, sync::Mutex};

use crate::primitives::{HashValue, NijikaRound, NijikaPBFTMessage};
use super::block::{DataBlockPool, NijikaTestControlBlock};

type NijikaMessagePool<'a> = Mutex<HashMap<HashValue, &'a NijikaPBFTMessage>>;
type PeerNodeMap = HashMap<HashValue, (String, String)>;

pub struct NijikaTestNode<'a> {
    name: String,
    ip: String,
    id: HashValue,
    ledger: Vec<NijikaTestControlBlock>,
    peer_nodes: PeerNodeMap,
    data_block_hash_queue: Vec<HashValue>,
    safe_data_block_pool: DataBlockPool<'a>,
    pbft_msg_hash_queue: Vec<HashValue>,
    safe_pbft_message_pool: NijikaMessagePool<'a>,
    nijika_round: NijikaRound,
    vrf_seed: u64,
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}