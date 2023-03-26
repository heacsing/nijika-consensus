mod node;
mod network;
mod conf;
mod block;

use node::NijikaTestNode;


async fn main() {
    let mut node = NijikaTestNode::new(19).expect("fail to create a new node");
    println!("running");
    node.start();
}