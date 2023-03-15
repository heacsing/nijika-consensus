use crate::primitives::{
    NijikaNodeRole,
    NijikaResult,
    NijikaPBFTMessage,
    NijikaPBFTMessageType,
    NijikaPBFTStage,
    NijikaError,
    HashValue
};

use super::NijikaPBFTStageApi;

pub trait NijikaPBFTMessageApi: NijikaPBFTStageApi {
    fn handle_pbft_message(&mut self, peer_id: HashValue, message: NijikaPBFTMessage) -> NijikaResult<()> {
        let message_type = message.get_type();
        let round_num = message.get_round_num();
        let message_source = message.get_source();
        let control_block_hash = message.get_control_block_hash();

        match message_type {
            NijikaPBFTMessageType::PrePrepare => {
                if let Some(control_block) = message.get_control_block() {
                    let message_hash = message.hash()?;
                    self.append_pbft_message_queue(message_hash)?;
                    self.insert_pbft_message_pool(message_hash, message)?;
                    if round_num != self.get_round_num() ||
                    self.get_round().get_stage() != NijikaPBFTStage::WaitPrePrepare ||
                    self.get_role() != NijikaNodeRole::VALIDATOR {
                        self.set_vrf_seed(control_block.get_seed())?;
                        self.set_round_control_block(control_block)?;
                    } else {
                        self.handle_pre_prepare(control_block)?;
                    }
                    self.broadcast_hash_message(message_hash, Some(peer_id))?;
                    Ok(())
                } else {
                    Err(NijikaError::InvalidControlBlock(format!("Missing control block from a message: {:#?}", message)))
                }
            }
            NijikaPBFTMessageType::Prepare => {
                if let Some(vote) = message.get_vote() {
                    let pbft_msg = NijikaPBFTMessage::new_vote_message(
                        message_source,
                        round_num,
                        message_type,
                        control_block_hash,
                        vote,
                    );
                    let pbft_msg_hash = pbft_msg.hash()?;
                    self.append_pbft_message_queue(pbft_msg_hash)?;
                    if round_num == self.get_round_num() &&
                    (self.get_role() == NijikaNodeRole::VALIDATOR ||
                    self.get_role() == NijikaNodeRole::PROPOSER) {
                        self.handle_prepare(vote.get_result())?;
                    }
                    self.broadcast_hash_message(pbft_msg_hash, Some(peer_id))?;
                    Ok(())
                } else {
                    Err(NijikaError::InvalidPBFTMessage(format!("An invalid pbft message with no nijika vote")))
                }
            },
            NijikaPBFTMessageType::Commit => {
                if let Some(vote) = message.get_vote() {
                    let pbft_msg = NijikaPBFTMessage::new_vote_message(
                        message_source,
                        round_num,
                        message_type,
                        control_block_hash,
                        vote,
                    );
                    let pbft_msg_hash = pbft_msg.hash()?;
                    self.append_pbft_message_queue(pbft_msg_hash)?;
                    if round_num == self.get_round_num() &&
                    (self.get_role() == NijikaNodeRole::VALIDATOR ||
                    self.get_role() == NijikaNodeRole::PROPOSER) {
                        self.handle_commit(vote.get_result())?;
                    }
                    self.broadcast_hash_message(pbft_msg_hash, Some(peer_id))?;
                    Ok(())
                } else {
                    Err(NijikaError::InvalidPBFTMessage(format!("An invalid pbft message with no nijika vote")))
                }
            },
            NijikaPBFTMessageType::Reply => {
                if let Some(control_block) = message.get_control_block() {
                    let message_hash = message.hash()?;
                    self.append_pbft_message_queue(message_hash)?;
                    if round_num == self.get_round_num() &&
                    (self.get_role() == NijikaNodeRole::PACKER
                    || self.get_role() == NijikaNodeRole::NORMAL
                    || self.get_round().get_stage() == NijikaPBFTStage::WaitReply) {
                        self.handle_reply(control_block)?;
                    }
                    self.broadcast_hash_message(message_hash, Some(peer_id))?;
                    Ok(())
                } else {
                    Err(NijikaError::InvalidControlBlock(format!("Missing control block from a message: {:#?}", message)))
                }
            },
            #[allow(unreachable_patterns)]
            _ => Err(NijikaError::InvalidPBFTMessage(format!("an invalid message containing an unknown type")))
        }
    }
}