use common::ClientPayload;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use std::error::Error;

pub struct Network {
    pub tx: mpsc::UnboundedSender<String>, // Send raw JSON strings to network task
    pub rx: mpsc::UnboundedReceiver<String>, // Receive raw JSON strings from network task
}

impl Network {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(addr).await?;
        let framed = Framed::new(stream, LinesCodec::new());
        let (mut sink, mut stream) = framed.split(); // `stream` here is the read half
        
        let (tx_out, mut rx_out) = mpsc::unbounded_channel::<String>();
        let (tx_in, rx_in) = mpsc::unbounded_channel::<String>();
        
        // Spawn writer task
        tokio::spawn(async move {
            while let Some(msg) = rx_out.recv().await {
                let _ = sink.send(msg).await;
            }
        });
        
        // Spawn reader task
        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                let _ = tx_in.send(msg);
            }
        });
        
        Ok(Self {
            tx: tx_out,
            rx: rx_in,
        })
    }
}
