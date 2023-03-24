mod message;
use std::{net::{TcpListener}, io::{self, Read, Write}};

use crate::node::NijikaTestNode;

pub struct Network {
    node: NijikaTestNode,
    client: Vec<TcpListener>,
}

impl Network {
    pub fn listen(&mut self, peer_ip: &str) {
        self.client.push(TcpListener::bind(peer_ip).unwrap());
        let listener = self.client.last().expect("wtf");
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = Vec::new();
                    let start_time = chrono::Utc::now().timestamp();
                    if let Ok(len) = stream.read_to_end(&mut buf) {
                        println!("recv from: {}", peer_ip);
                    }

                },
                Err(e) => {println!("error while listening: {}", e);}
            }
        }
    }
}