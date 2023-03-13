use std::rc::Rc;

use serde::Serialize;

use super::{HashValue, NijikaRound, NijikaControlBlockT, NijikaBlockT, NijikaResult, NijikaPBFTMessage, NijikaPBFTStage, NijikaError, NijikaDataBlockT};

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum NijikaNodeType {
    NORMAL,
    PACKER,
    PROPOSER,
    VALIDATOR,
}

pub trait NijikaNodeT {
    fn check(&self, role: NijikaNodeType, stage: NijikaPBFTStage) -> NijikaResult<()> {
        let current_round = self.get_round();
        let current_role = current_round.get_role();
        let current_stage = current_round.get_stage();
        if role != current_role {
            return Err(NijikaError::MismatchedRole(current_role, role));
        }
        if stage != current_stage {
            return Err(NijikaError::MismatchedStage(current_stage, stage));
        }
        Ok(())
    }
    // basic info
    fn get_name(&self) -> &str;

    fn get_ip(&self) -> &str;

    fn get_id(&self) -> &HashValue;

    fn set_vrf_seed(&mut self, seed: u64) -> NijikaResult<()>;



    // pbft round info
    fn get_round(&self) -> &NijikaRound;

    fn get_round_mut(&self) -> &mut NijikaRound;

    fn get_round_num(&self) -> u64;

    /// set the control_block field of node's PBFTRound with the given block.
    fn set_round_control_block(&mut self, block: Rc<dyn NijikaControlBlockT>) -> NijikaResult<()>;

    fn get_round_control_block(&mut self) -> Rc<dyn NijikaControlBlockT>;

    fn end_round(&self) -> NijikaResult<()>;

    fn try_end_round(&mut self) -> NijikaResult<()>;

    fn set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()>;

    fn try_set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()>;



    // handle block, block queue and block pool
    /// Create a new control block with pre_hash.
    /// Make sure that its seed equals node's VRFSeed.
    /// Then, fill its data_block_pointers and empty the node's data_block_hash_queue
    /// Finally, sign the block with node's key
    fn new_control_block(&mut self) -> Rc<dyn NijikaControlBlockT>;
    /// Create a new data block
    fn new_data_block(&mut self) -> Rc<dyn NijikaDataBlockT>;

    /// append the given hash to the node's data block hash queue
    fn append_data_block_hash_queue(&mut self, hash: HashValue) -> NijikaResult<()>;

    /// use the given hash as Key, the block as Value. Then insert it into the node's data block pool
    fn insert_data_block_pool(&mut self, hash: HashValue, block: Rc<dyn NijikaDataBlockT>) -> NijikaResult<()>;



    // handle pbft message
    /// append node's pbft_message_queue with the given hash value
    fn append_pbft_message_queue(&mut self, hash: HashValue) -> NijikaResult<()>;

    /// use the given hash as Key, the message as Value. Then insert it into the pbft_message_pool
    fn insert_pbft_message_pool(&mut self, hash: HashValue, message: NijikaPBFTMessage) -> NijikaResult<()>;

    /// Depending on the node's impl. broadcast the hash
    fn broadcast_message_hash(&self, hash: HashValue) -> NijikaResult<()>;
}