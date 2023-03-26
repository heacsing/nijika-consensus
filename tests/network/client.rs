use libp2p::{
    swarm::{NetworkBehaviourEventProcess},
    NetworkBehaviour, gossipsub, mdns
};
use nijika::{NijikaResult, NijikaError};
use tokio::sync::mpsc;
use std::{collections::hash_map::DefaultHasher, error::Error, sync::Arc};
use std::hash::{Hash, Hasher};
use std::time::Duration;
use crate::{node::NijikaTestNode, conf::{CHAIN_TOPIC, BLOCK_TOPIC}, NodeEvent, Event};

use super::Message;

#[derive(NetworkBehaviour)]
pub struct Client {
    pub gossipsub: gossipsub::Gossipsub,
    pub mdns: mdns::Mdns,
    #[behaviour(ignore)]
    pub sender: mpsc::UnboundedSender<Event>,
    #[behaviour(ignore)]
    pub message_pool: Vec<gossipsub::MessageId>
}


impl Client {
    pub async fn new(
        node: &NijikaTestNode,
        sender: mpsc::UnboundedSender<Event>,
    ) -> NijikaResult<Self> {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(5))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(|m: &gossipsub::GossipsubMessage| {
                let mut s = DefaultHasher::new();
                m.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            })
            .build()
            .expect("fail to build gossipsub config");
        let gossipsub = gossipsub::Gossipsub::new(gossipsub::MessageAuthenticity::Signed(node.get_key().clone()), gossipsub_config).or(Err(NijikaError::InitializeFailed))?;
        let mut client = Self {
            gossipsub: gossipsub,
            mdns: mdns::Mdns::new(Default::default()).await.expect("fail to instance mdns"),
            sender,
            message_pool: vec![],
        };
        client.gossipsub.subscribe(&CHAIN_TOPIC).or(Err(NijikaError::InitializeFailed))?;
        client.gossipsub.subscribe(&BLOCK_TOPIC).or(Err(NijikaError::InitializeFailed))?;

        Ok(client)
    }
}

impl NetworkBehaviourEventProcess<gossipsub::GossipsubEvent> for Client {
    fn inject_event(&mut self, event: gossipsub::GossipsubEvent) {
        match event {
            gossipsub::GossipsubEvent::Message { propagation_source, message_id, message } => {
                if !self.message_pool.contains(&message_id) {
                    self.message_pool.push(message_id);
                    if let Ok(msg) = bincode::deserialize::<Message>(&message.data) {
                        self.sender.send(Event::Message(msg)).unwrap();
                    }
                }
            },
            _ => {}
        }
    }
}

impl NetworkBehaviourEventProcess<mdns::MdnsEvent> for Client {
    fn inject_event(&mut self, event: mdns::MdnsEvent) {
        match event {
            mdns::MdnsEvent::Discovered(list) => {
                for (peer, _addr) in list {
                    self.gossipsub.add_explicit_peer(&peer);
                }
            }
            mdns::MdnsEvent::Expired(list) => {
                for (peer, _addr) in list {
                    self.gossipsub.remove_explicit_peer(&peer);
                }
            }
        }
    }
}
