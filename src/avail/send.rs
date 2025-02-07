use avail_rust::{Data, Keypair, SecretUri, WaitFor, SDK};
use core::str::FromStr;
use std::error::Error;
use bincode;
use crate::models::batch::Batch;

pub async fn send(batch: Batch) -> Result<(String, String), Box<dyn Error>> {

    let rpc = std::env::var("AVAIL_RPC")
        .map_err(|e| format!("Failed to get AVAIL_RPC environment variable: {}", e))?;

    let sdk = SDK::new(&rpc)
        .await
        .map_err(|e| format!("Failed to initialize SDK: {}", e))?;

    let secret_uri = SecretUri::from_str(
        std::env::var("SEED").unwrap().as_str()
    ).unwrap();

    let account = Keypair::from_uri(&secret_uri).unwrap();
    let bytes = bincode::serialize(&batch).unwrap();
    let data = Data { 0: bytes };

    let result = sdk
        .tx
        .data_availability
        .submit_data(data, WaitFor::BlockFinalization, &account, None)
        .await?;

    println!("result.block_hash={:?}", result.block_hash);
    println!("result.tx_hash={:?}", result.tx_hash);

    Ok((result.block_hash.to_string(), result.tx_hash.to_string()))
}