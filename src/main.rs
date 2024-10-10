use anyhow::Result;
use futures::prelude::*;
use log::{debug, info};
use redis::Client;
use sysinfo::System;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut sys = System::new_all();
    sys.refresh_all();

    let client = Client::open("rediss://localhost:6700".to_string())?;
    let pchannels = vec!["test*".to_string()];

    let mut sub_conn = client.get_async_pubsub().await?;
    for chan in pchannels.iter() {
        info!("psubscribing event {}", chan);
        sub_conn.psubscribe(chan).await?;
    }

    let mut sub_stream = sub_conn.on_message();

    loop {
        let start = Instant::now();
        let next_msg = sub_stream.next().await;
        match next_msg {
            Some(msg) => {
                sys.refresh_memory();
                sys.refresh_cpu_usage();
                info!(
                    "got message on channel: {}, size: {}KB, time: {:?} cpu: {} mem: {}MB",
                    msg.get_channel_name(),
                    msg.get_payload_bytes().len() / 1024,
                    start.elapsed(),
                    sys.global_cpu_usage(),
                    sys.used_memory() / 1024 / 1024,
                );
                // info!("got message with size: {}", msg.get_payload_bytes().len());
                // debug!("new event: {}", msg.get_channel_name());
            }
            None => {
                info!("Next returns NONE, somehow stream is disconnected");
                return Ok(());
            }
        }
    }
}
 
 

        

        
use std::time::Instant;

