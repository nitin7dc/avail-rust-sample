mod avail;
mod models;

use dotenv::dotenv;
use core::str::FromStr;
use std::error::Error;
use bincode;
use serde_json::json;
use crate::models::batch::Batch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let batch = Batch {
        batch_number: "1".to_string()
    };

    let (block_hash, tx_hash) = avail::send(batch).await?;
    println!("block_hash: {}, tx_hash: {}", block_hash, tx_hash);

    Ok(())
}