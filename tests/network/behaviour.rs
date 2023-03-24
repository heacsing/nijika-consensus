use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use futures::channel::mpsc;
use libp2p::{gossipsub, mdns, swarm::{NetworkBehaviour}};


#[derive(NetworkBehaviour)]
pub struct NijikaTestBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::async_io::Behaviour,
}

impl NijikaTestBehaviour {
    pub fn new(gossipsub: gossipsub::Behaviour, mdns: mdns::async_io::Behaviour) -> Self {
        Self {gossipsub, mdns}
    }
    pub fn id(message: &gossipsub::Message) -> gossipsub::MessageId {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    }
}