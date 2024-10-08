use anyhow::Result;
use futures::prelude::*;
use log::{debug, info};
use redis::Client;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let client = Client::open("rediss://localhost:6700".to_string())?;
    let pchannels = vec!["test*".to_string()];

    let mut sub_conn = client.get_async_pubsub().await?;
    for chan in pchannels.iter() {
        info!("psubscribing event {}", chan);
        sub_conn.psubscribe(chan).await?;
    }

    let mut sub_stream = sub_conn.on_message();

    loop {
        let next_msg = sub_stream.next().await;
        match next_msg {
            Some(msg) => {
                info!("got message with size: {}", msg.get_payload_bytes().len());
                debug!("new event: {}", msg.get_channel_name());
            }
            None => {
                info!("Next returns NONE, somehow stream is disconnected");
                return Ok(());
            }
        }
    }
}

 
 

        

        
