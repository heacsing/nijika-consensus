use crate::primitives::{
    NijikaResult,
    NijikaNodeT,
    NijikaNodeType,
    NijikaPBFTStage,
    NijikaPBFTMessage,
    NijikaMessageType
};

pub fn reply(mut node: Box<dyn NijikaNodeT>) -> NijikaResult<()> {
    node.check(NijikaNodeType::PROPOSER, NijikaPBFTStage::REPLY)?;
    let control_block = node.get_round_control_block();
    let control_block_hash = control_block.hash()?;
    let pbft_msg = NijikaPBFTMessage::new_vote_message(
        node.get_id().clone(),
        node.get_round_num(),
        NijikaMessageType::PREPARE,
        control_block_hash
    );
    let pbft_msg_hash = pbft_msg.hash()?;
    node.append_pbft_message_queue(pbft_msg_hash)?;
    node.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
    node.broadcast_message_hash(pbft_msg_hash)?;
    println!("[Round Completed]");
    node.end_round()?;
    Ok(())
}