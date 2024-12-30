use crate::encode_decode::Decodable;

use crate::error::Error;
use serde::Serialize;

#[derive(Debug)]
pub struct RawTransaction(Vec<u8>);

#[derive(Debug)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub lock_time: u32,
}
#[derive(Debug)]
pub struct TxIn {
    pub tx_id: TxID,
    pub output_index: u32,
    pub script_sig: String,
    pub witness: Witness,
    pub sequence: u32,
}

#[derive(Debug)]
pub struct Witness {
    pub content: Vec<Vec<u8>>,
}
#[derive(Debug)]
pub struct TxOut {
    pub amount: Amount,
    pub script_pubkey: String,
}

#[derive(Debug)]
pub struct Amount(pub u64);

#[derive(Debug)]
pub struct TxID(pub [u8; 32]);

#[derive(Debug, Serialize)]
pub struct CompactSize(pub u64);

impl RawTransaction {
    pub fn new(tx_hex: String) -> Result<RawTransaction, hex::FromHexError> {
        hex::decode(tx_hex).map(|v| RawTransaction(v))
    }
}

impl Transaction {
    pub fn new(raw_tx: &RawTransaction) -> Result<Transaction, Error> {
        Transaction::decode(&mut raw_tx.0.as_slice())
    }
}

impl Amount {
    pub fn from_sat(satoshi: u64) -> Amount {
        Amount(satoshi)
    }
    pub fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

impl Witness {
    pub fn new() -> Self {
        Witness { content: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}
