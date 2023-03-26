mod message;
pub use message::*;
use nijika::{NijikaResult, NijikaError};
use tokio::{spawn, net::TcpStream, io::AsyncWriteExt};

pub fn tcp_send(bytes: Vec<u8>, target: String) {
    spawn(async move {
        let mut stream = TcpStream::connect(target).await.unwrap();
        if let Ok(_) = stream.writable().await {
            stream.write_all(&bytes).await.unwrap();
        }
    });
}
