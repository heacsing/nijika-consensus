mod node;
mod network;
mod conf;
mod block;

use std::{time::Duration, collections::HashSet, sync::Arc};

use node::NijikaTestNode;

use network::{Client, Message};

use libp2p::{
    mplex::MplexConfig,
    tcp::TokioTcpConfig,
    core::upgrade,
    noise::{
        NoiseConfig,
        Keypair,
        X25519Spec,
    }, swarm::SwarmBuilder, Transport,
};
use tokio::{sync::mpsc, spawn, time::sleep, select};

#[derive(Debug)]
pub enum Event {
    Init,
    Message(Message),
    NodeEvent(NodeEvent),
}
#[derive(Debug)]
pub enum NodeEvent {

}

async fn main() {
    pretty_env_logger::init();
    // let (node_sender, mut node_rcv) = mpsc::unbounded_channel::<NodeEvent>();
    let (sender, mut recver) = mpsc::unbounded_channel::<Event>();
    let node = NijikaTestNode::new(10, sender.clone()).expect("unable to initialize a new node");

    // let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
    // let (init_sender, mut init_rcv) = mpsc::unbounded_channel();
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(node.get_key())
        .expect("cannot generate authentic keys");
    let peer_id = node.get_peer_id().unwrap();

    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(MplexConfig::new())
        .boxed();

    let client = Client::new(&node, sender.clone()).await.unwrap();

    let mut swarm = SwarmBuilder::new(transp, client, peer_id)
        .executor(Box::new(|fut| {
            spawn(fut);
        }))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().expect("local socket")).expect("swarm failed");
    let init_sender = sender.clone();
    spawn(async move {
        sleep(Duration::from_secs(1)).await;
        init_sender.send(Event::Init).expect("init msg failed");
    });

    loop {
        let event = {
            select! {
                Some(e) = recver.recv() => {
                    e
                }
            }
        };
        match event {
            Event::Init => {
                let peers = swarm.behaviour().mdns.discovered_nodes();
                let mut unique_set = HashSet::new();
                for peer in peers {
                    unique_set.insert(peer);
                }
                let peers: Vec<String> = unique_set.iter().map(|p| p.to_string()).collect();
                
            },
            Event::Message(msg) => {
                
            },
            Event::NodeEvent(e) => todo!(),
        }
        
    };
}

