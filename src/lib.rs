use std::error::Error;

pub fn decode(tx_hex: String) -> Result<String, Box<dyn Error>> {
    let tx_bytes = hex::decode(tx_hex).map_err(|err| format!("Hex decode error: {}", err))?;
    Ok(serde_json::to_string_pretty(&tx_bytes)?)
}
pub mod error;
pub mod types;
