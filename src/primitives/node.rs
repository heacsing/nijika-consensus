use std::{collections::HashMap, fmt::Debug};

use serde::Serialize;

use super::{HashValue, NijikaRound, NijikaControlBlockT, NijikaResult, NijikaPBFTMessage, NijikaPBFTStage, NijikaError, NijikaDataBlockT, NijikaPBFTMessageType};

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy, Hash)]
pub enum NijikaNodeRole {
    NORMAL,
    PACKER,
    PROPOSER,
    VALIDATOR,
}

pub trait NijikaNodeT<'a, CB: NijikaControlBlockT, DB: NijikaDataBlockT, ID: Clone + Copy + Debug + Serialize> {
    // basic info
    fn get_name(&self) -> &str;

    fn get_ip(&self) -> &str;

    fn get_id(&self) -> ID;

    fn get_role(&self) -> NijikaNodeRole;

    fn get_weight(&self) -> u64;
    fn get_total_weight(&self) -> u64;
    fn get_vrf_params(&self) -> (u64, u64);

    fn get_peer_info_mut(&mut self) -> &mut HashMap<HashValue, (String, String)>;

    fn get_hash_queue(&self, identifier: Option<&str>) -> NijikaResult<&Vec<HashValue>>;
    fn get_hash_queue_mut(&mut self, identifier: Option<&str>) -> NijikaResult<&mut Vec<HashValue>>;

    fn get_vrf_seed(&self) -> u64;
    fn set_vrf_seed(&mut self, seed: u64) -> ();

    fn get_secret_key(&self) -> &[u8];
    fn get_public_key(&self) -> &[u8];
    fn set_keys(&mut self, private_key: Vec<u8>, public_key: Vec<u8>) -> ();
    fn update_proof(&mut self, proof: Vec<u8>, hash: Vec<u8>) -> NijikaResult<()>;

    // pbft round info

    fn set_round(&mut self, round: NijikaRound<CB>) -> NijikaResult<()>;

    fn get_round(&self) -> &NijikaRound<CB>;

    fn get_round_mut(&mut self) -> &mut NijikaRound<CB>;

    fn get_round_num(&self) -> u64;

    /// set the control_block field of node's PBFTRound with the given block.
    fn set_round_control_block(&mut self, block: CB) -> NijikaResult<()>;

    fn get_round_control_block(&mut self) -> &CB;

    // fn end_round(&mut self) -> NijikaResult<()>;

    // fn try_end_round(&mut self) -> NijikaResult<()>;

    // fn set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()>;

    // fn try_set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()>;



    // handle block, block queue and block pool
    /// Create a new control block with pre_hash.
    /// Make sure that its seed equals node's VRFSeed.
    /// Then, fill its data_block_pointers and empty the node's data_block_hash_queue
    /// Finally, sign the block with node's key
    fn new_control_block(&self) -> CB;
    fn load_control_block(&mut self, block: &mut CB) -> NijikaResult<()>;
    fn commit_control_block(&mut self, block: CB) -> NijikaResult<()>;

    /// Create a new data block
    fn new_data_block(&self) -> DB;

    /// append the given hash to the node's data block hash queue
    fn append_data_block_hash_queue(&mut self, hash: HashValue) -> NijikaResult<()>;

    /// use the given hash as Key, the block as Value. Then insert it into the node's data block pool
    fn insert_data_block_pool(&mut self, hash: HashValue, block: DB) -> NijikaResult<()>;



    // handle pbft message
    /// append node's pbft_message_queue with the given hash value
    fn append_pbft_message_queue(&mut self, hash: HashValue) -> NijikaResult<()>;

    /// use the given hash as Key, the message as Value. Then insert it into the pbft_message_pool
    fn insert_pbft_message_pool(&mut self, hash: HashValue, message: NijikaPBFTMessage<CB, ID>) -> NijikaResult<()>;

    /// create a inv message and then broadcast the given hash to all peers, except the source node
    fn broadcast_hash_message(&self, hash: HashValue, source: Option<HashValue>) -> NijikaResult<()>;

}