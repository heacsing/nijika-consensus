use async_std::io;
use futures::{prelude::*, select, channel::mpsc};
use libp2p::{
    gossipsub, identity, mdns,
    swarm::NetworkBehaviour,
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId,
};
use std::{collections::hash_map::DefaultHasher, error::Error};
use std::hash::{Hash, Hasher};
use std::time::Duration;
use crate::{node::NijikaTestNode, network::behaviour::NijikaTestBehaviour};

impl NijikaTestNode {
    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        println!("***********************Nijika***********************");
        println!("{:#?}", &self);
        println!("*********************Node Info**********************");
        let mut round_num = 1u64;
        let local_key = self.get_key();
        if let Ok(local_peer_id) = self.get_peer_id() {let transport = libp2p::development_transport(local_key.clone()).await?;

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(2))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(|m: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    m.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                })
                .build()
                .expect("invalid config");
    
            let mut gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(local_key.clone()),
                gossipsub_config
            ).expect("Incorrect Configuration");

            gossipsub.subscribe(self.get_topic())?;

            let mut swarm = {
                let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)?;
                let behaviour = NijikaTestBehaviour::new(gossipsub, mdns);
                SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
            };
            swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

            loop {
                select! {
                    event = swarm.select_next_some() => match event {
                        SwarmEvent::Behaviour() => todo!(),
                        _ => {}
                    }
                }
            }
        } else {
            Ok(())
        }
    }
}