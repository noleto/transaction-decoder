use std::error::Error;

use types::{RawTransaction, Transaction};

pub fn decode(tx_hex: String) -> Result<String, Box<dyn Error>> {
    Ok(RawTransaction::new(tx_hex)
        .map_err(|err| format!("Hex decode error: {}", err))
        .map(|raw_tx| Transaction::new(&raw_tx))?
        .map(|tx| serde_json::to_string_pretty(&tx))??)
}
pub mod encode_decode;
pub mod error;
pub mod serialization;
pub mod transaction;
pub mod types;
