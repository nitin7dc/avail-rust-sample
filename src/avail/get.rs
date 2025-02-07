use avail_rust::{avail, SDK};
use core::str::FromStr;
use std::error::Error;
use avail::data_availability::calls::types as DataAvailabilityCalls;
use avail_rust::avail::runtime_types::sp_core;
use serde_json::Error as SerdeError;
use crate::models::batch::Batch;

pub async fn get(tx_hash: &str, block_hash: &str) -> Result<(Batch), Box<dyn Error>> {

    let rpc = std::env::var("AVAIL_RPC")
        .map_err(|e| format!("Failed to get AVAIL_RPC environment variable: {}", e))?;

    let sdk = SDK::new(&rpc)
        .await
        .map_err(|e| format!("Failed to initialize SDK: {}", e))?;

    let block_hash = sp_core::H256::from_str(block_hash)
        .map_err(|e| format!("Invalid block hash format: {}", e))?;

    let tx_hash = sp_core::H256::from_str(tx_hash)
        .map_err(|e| format!("Invalid tx hash format: {}", e))?;

    let tx = sdk
        .util
        .fetch_transaction::<DataAvailabilityCalls::SubmitData>(block_hash, tx_hash)
        .await
        .map_err(|err| format!("Failed to fetch transaction: {:?}", err))?;

    let payload = tx.value.data.0;

    // Deserialize the payload into the Batch struct
    let batch: Batch = serde_json::from_slice(&payload)
        .map_err(|e: SerdeError| format!("Failed to deserialize Batch: {}", e))?;

    Ok(batch)
}