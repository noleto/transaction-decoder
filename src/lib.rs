use crate::encode_decode::Decodable;
use std::error::Error;

use types::Transaction;

pub fn decode(tx_hex: String) -> Result<String, Box<dyn Error>> {
    let tx_bytes = hex::decode(tx_hex).map_err(|err| format!("Hex decode error: {}", err))?;
    let tx = Transaction::decode(&mut tx_bytes.as_slice())?;

    Ok(serde_json::to_string_pretty(&tx)?)
}
pub mod encode_decode;
pub mod error;
pub mod serialization;
pub mod transaction;
pub mod types;
