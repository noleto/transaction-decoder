use crate::error::Error;
use std::io::BufRead;

#[derive(Debug)]
pub struct Transaction {
    version: u32,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    lock_time: u32,
}
#[derive(Debug)]
pub struct TxIn {
    tx_id: TxID,
    output_index: u32,
    script_sig: String,
    witness: Witness,
    sequence: u32,
}

#[derive(Debug)]
pub struct Witness {
    content: Vec<Vec<u8>>,
}
#[derive(Debug)]
pub struct TxOut {
    amount: Amount,
    script_pubkey: String,
}

#[derive(Debug)]
pub struct Amount(u64);

#[derive(Debug)]
pub struct TxID([u8; 32]);

pub trait Decodable: Sized {
    fn decode<R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>;
}
