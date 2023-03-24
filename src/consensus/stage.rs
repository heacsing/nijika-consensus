use std::fmt::Debug;

use rug::Integer;
use serde::Serialize;

use crate::{primitives::{
    NijikaNodeT,
    NijikaNodeRole,
    NijikaResult,
    NijikaPBFTMessage,
    NijikaPBFTMessageType,
    NijikaPBFTStage,
    NijikaControlBlockT,
    NijikaError,
    NijikaVote,
    NijikaRound,
    NijikaDataBlockT
}, vrf::{self, NijikaVRFParams, NijikaVRFClientS}};

pub trait NijikaPBFTStageApi<'a, CB: NijikaControlBlockT + Serialize + Debug + Clone + 'a, DB: NijikaDataBlockT + Serialize + Debug + Clone + 'a, ID: Clone + Copy + Debug + Serialize  + 'a>: NijikaNodeT<'a, CB, DB, ID> {
    fn vrf_selection (&mut self) -> NijikaResult<NijikaNodeRole> {
        let (expected, total) = self.get_vrf_params();
        let mut vrf_client = NijikaVRFClientS::new(self.get_weight(), expected, total);
        let seed = self.get_vrf_seed();
        // e.g. {NijikaNodeRole::Packer: (vrf_hash, vrf_proof)}
        let role_keys = [NijikaNodeRole::PACKER, NijikaNodeRole::PROPOSER, NijikaNodeRole::VALIDATOR];
        // let mut role_map : HashMap<NijikaNodeRole, (Vec<u8>, Vec<u8>)>= HashMap::new();
        for role in role_keys {
            let params = NijikaVRFParams {
                weight: self.get_weight(),
                round: self.get_round_num(),
                seed: seed,
                role
            };
            if let Ok((proof, hash)) = vrf_client.prove(self.get_secret_key(), &params) {
                // role_map.insert(role, (hash, proof));
                // let hash_value: Float = Integer::from_digits(&hash, rug::integer::Order::Lsf) / Integer::i_pow_u(2, 256);
                let (index, _) = vrf_client.sortition(&hash);
                if index > 0 {
                    match self.update_proof(proof, hash) {
                        Ok(_) => {
                            return Ok(role);
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            } else {
                return Err(NijikaError::VRFError(format!("Error when generating node's hash and proof in role: {:?}", role)));
            }
        }
        Ok(NijikaNodeRole::NORMAL)
    }
    fn start_a_new_round(&mut self, round_num: u64, thresh: u64, expected: u64) -> NijikaResult<()> {
        let role = self.vrf_selection()?;
        match role {
            NijikaNodeRole::NORMAL => self.set_round(NijikaRound::new(thresh, expected, round_num, role, NijikaPBFTStage::WaitReply)),
            NijikaNodeRole::PACKER => self.set_round(NijikaRound::new(thresh, expected, round_num, role, NijikaPBFTStage::Packing)),
            NijikaNodeRole::VALIDATOR => self.set_round(NijikaRound::new(thresh, expected, round_num, role, NijikaPBFTStage::WaitPrePrepare)),
            NijikaNodeRole::PROPOSER => {
                self.set_round(NijikaRound::new(thresh, expected, round_num, role, NijikaPBFTStage::PrePrepare))?;
                self.prepare()
            }
        }
    }

    fn commit_round(&mut self) -> NijikaResult<()> {
        let block = self.get_round().get_control_block().expect("empty block in the round");
        self.commit_control_block(block.clone())
    }

    fn end_round(&mut self) -> NijikaResult<()> {
        self.get_round_mut().end();
        Ok(())
    }

    fn try_end_round(&mut self) -> NijikaResult<()> {
        self.end_round()
    }

    fn set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()> {
        self.get_round_mut().set_stage(next);
        Ok(())
    }

    fn try_set_stage(&mut self, next: NijikaPBFTStage) -> NijikaResult<()> {
        let next = self.get_round_mut().try_set_stage(next);
        match next {
            Ok(stage) => {
                if stage == NijikaPBFTStage::Commit {
                    self.commit()
                } else if stage == NijikaPBFTStage::Reply {
                    self.commit_round()?;
                    self.reply()
                } else {
                    println!("stage error: cannot enter next stage");
                    Ok(())
                }
            }
            Err(e) => {
                println!("stage error: {:#?}", e);
                Ok(())
            }
        }
    }

    fn check(&self, role: NijikaNodeRole, stage: NijikaPBFTStage) -> NijikaResult<()> {
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
    fn pre_prepare(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PROPOSER, NijikaPBFTStage::PrePrepare)?;
        let control_block = self.new_control_block();
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_control_block_message(
            self.get_id(),
            self.get_round_num(),
            NijikaPBFTMessageType::PrePrepare,
            control_block_hash,
            control_block.clone()
        );
        self.set_round_control_block(control_block)?;
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.set_stage(NijikaPBFTStage::Prepare)?;
        println!("[Complete PrePrepare]");
        Ok(())
    }
    fn handle_pre_prepare(&mut self, control_block: CB) -> NijikaResult<()> {
        println!("[Handle PrePrepare]");
        self.set_vrf_seed(control_block.get_seed());
        self.set_round_control_block(control_block)?;
        self.set_stage(NijikaPBFTStage::Prepare)?;
        self.prepare()
    }




    fn prepare(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::VALIDATOR, NijikaPBFTStage::Prepare)?;
        let control_block = self.get_round_control_block();
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_vote_message(
            self.get_id(),
            self.get_round_num(),
            NijikaPBFTMessageType::Prepare,
            control_block_hash,
            NijikaVote::new_true(self.get_id())
        );
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.broadcast_hash_message(pbft_msg_hash, None)?;
        self.try_set_stage(NijikaPBFTStage::Commit)?;
        Ok(())
    }
    fn handle_prepare(&mut self, vote_result: bool) -> NijikaResult<()> {
        println!("[Handle Prepare]");
        if vote_result {
            let current_round = self.get_round_mut();
            current_round.vote_inc(NijikaPBFTStage::Prepare)?;
        }
        self.try_set_stage(NijikaPBFTStage::Commit)
    }




    fn commit(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PROPOSER, NijikaPBFTStage::Commit)?;
        let control_block = self.get_round_control_block();
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_vote_message(
            self.get_id(),
            self.get_round_num(),
            NijikaPBFTMessageType::Commit,
            control_block_hash,
            NijikaVote::new_true(self.get_id())
        );
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.broadcast_hash_message(pbft_msg_hash, None)?;
        self.try_set_stage(NijikaPBFTStage::Reply)?;
        Ok(())
    }
    fn handle_commit(&mut self, vote_result: bool) -> NijikaResult<()> {
        println!("[Handle Commit]");
        if vote_result {
            let current_round = self.get_round_mut();
            current_round.vote_inc(NijikaPBFTStage::Commit)?;
        }
        self.try_set_stage(NijikaPBFTStage::Reply)
    }




    fn reply(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PROPOSER, NijikaPBFTStage::Reply)?;
        let control_block = self.get_round_control_block().clone();
        let control_block_hash = control_block.hash()?;
        let pbft_msg = NijikaPBFTMessage::new_control_block_message(
            self.get_id().clone(),
            self.get_round_num(),
            NijikaPBFTMessageType::Prepare,
            control_block_hash,
            control_block
        );
        let pbft_msg_hash = pbft_msg.hash()?;
        self.append_pbft_message_queue(pbft_msg_hash)?;
        self.insert_pbft_message_pool(pbft_msg_hash, pbft_msg)?;
        self.broadcast_hash_message(pbft_msg_hash, None)?;
        println!("[Round Completed]");
        self.end_round()?;
        Ok(())
    }
    fn handle_reply(&mut self, control_block: &'a CB) -> NijikaResult<()> {
        println!("[Handle Reply]");
        if control_block.hash()? == self.get_round_control_block().hash()? {
            let current_round = self.get_round_mut();
            current_round.vote_inc(NijikaPBFTStage::Reply)?;
        }
        self.try_end_round()
    }




    fn pack(&mut self) -> NijikaResult<()> {
        self.check(NijikaNodeRole::PACKER, NijikaPBFTStage::Packing)?;
        let data_block = self.new_data_block();
        let data_block_hash = data_block.hash()?;
        self.append_data_block_hash_queue(data_block_hash)?;
        self.insert_data_block_pool(data_block_hash, data_block)?;
        self.broadcast_hash_message(data_block_hash, None)?;
        self.set_stage(NijikaPBFTStage::WaitReply)?;
        println!("[Complete Pack]");
        Ok(())
    }
}
