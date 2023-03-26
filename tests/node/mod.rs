mod event;
mod implementation;

use std::{collections::HashMap};

use nijika::{HashValue, NijikaRound, NijikaPBFTMessage, NijikaError, NijikaResult, NijikaNodeRole, NijikaVRFClientS, NijikaNodeT, NijikaBlockT, NijikaPBFTStageApi};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio::{spawn, select};

use crate::block::{DataBlockPool, NijikaTestControlBlock};
use crate::conf::{TotalWeights};
use crate::network::{Message, MessageType, tcp_send};

use self::event::Event;

type NijikaMessagePool = HashMap<HashValue, NijikaPBFTMessage<NijikaTestControlBlock, HashValue>>;
type PeerNodeMap = HashMap<HashValue, (String, String)>;


#[derive(Debug)]
pub struct NijikaTestNode {
    name: String,
    ip: String,
    id: HashValue,
    /* key: identity::Keypair,
    topic: IdentTopic, */
    moneys: Vec<u64>,
    total_weight: u64,
    ledger: Vec<NijikaTestControlBlock>,
    peer_nodes: PeerNodeMap,
    data_block_hash_queue: Vec<HashValue>,
    safe_data_block_pool: DataBlockPool,
    pbft_msg_hash_queue: Vec<HashValue>,
    safe_pbft_message_pool: NijikaMessagePool,
    nijika_round: NijikaRound<NijikaTestControlBlock>,
    vrf_seed: u64,
    vrf_proof: Vec<u8>,
    vrf_hash: Vec<u8>,
    vrf_public_key: Vec<u8>,
    vrf_secret_key: Vec<u8>,
    channel: (UnboundedSender<Event>, UnboundedReceiver<Event>),
}

impl NijikaTestNode {
    pub fn new(seed: u64) -> Option<Self> {
        let mut vrf_client = NijikaVRFClientS::new_raw();
        if let Ok((p1, p2)) = vrf_client.gen_keys(seed) {
            let rndm = rand::random::<u64>();
            Some(Self {
                name: format!("nijika-node-{}", rndm),
                ip: String::from("127.0.0.1:13000"),
                id: HashValue::random(),
                moneys: vec![1000],
                total_weight: TotalWeights,
                ledger: vec![],
                peer_nodes: PeerNodeMap::new(),
                data_block_hash_queue: vec![],
                safe_data_block_pool: DataBlockPool::new(),
                pbft_msg_hash_queue: vec![],
                safe_pbft_message_pool: NijikaMessagePool::new(),
                nijika_round: NijikaRound::default(),
                vrf_seed: rndm,
                vrf_hash: vec![],
                vrf_proof: vec![],
                vrf_public_key: p2,
                vrf_secret_key: p1,
                channel: mpsc::unbounded_channel(),
            })
        } else {
            None
        }
    }
    fn genesis(&mut self) -> NijikaResult<()> {
        let genesis = self.new_control_block();
        self.commit_control_block(genesis);
        let db = self.new_data_block();
        let hash = db.hash()?;
        self.append_data_block_hash_queue(hash)?;
        self.insert_data_block_pool(hash, db)?;
        Ok(())
    }
    fn handle_message(&mut self, m: Message) {
        let msg_type = m.get_type();
        match msg_type {
            MessageType::Invite => todo!(),
            MessageType::GetData => todo!(),
            MessageType::Data => todo!(),
        }
    }
    #[tokio::main]
    pub async fn start(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:10019").await.unwrap();
        let network_sender = self.channel.0.clone();
        spawn(async move {
            let loop_sender = network_sender.clone();
            loop {
                let owned_sender = loop_sender.clone();
                let (socket, _) = listener.accept().await.unwrap();
                spawn(async move {
                    socket.readable().await.unwrap();
                    let mut buf = [0u8; 2048];
                    socket.try_read(&mut buf).unwrap();
                    let msg = bincode::deserialize::<Message>(&buf).unwrap();
                    owned_sender.send(Event::IncomingMessage(msg)).unwrap();
                });
            }
        });
        self.genesis().unwrap();
        let mut round_num = 1;
        loop {
            self.start_a_new_round(round_num, 3, 3).unwrap();
            round_num += 1;
            select! {
                Some(e) = self.channel.1.recv() => {
                    match e {
                        Event::IncomingMessage(m) => {
                            self.handle_message(m);
                        },
                        Event::RoundEnd(n) => {
                            if round_num != n {
                                panic!("wtf");
                            } else {
                                println!("round end with {:#?}", self.get_round_control_block())
                            }
                        },
                        Event::OutgoingMessage(t, m) => {
                            let bytes = bincode::serialize(&m).unwrap();
                            tcp_send(bytes, t)
                        }
                    }
                }
            }
        }
    }
}

