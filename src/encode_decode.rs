use crate::{
    error::Error,
    types::{Amount, CompactSize, Transaction, TxID, TxIn, TxOut, Witness},
};
use std::io::{BufRead, Write};
pub trait Decodable: Sized {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized;
}

pub trait Encodable {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error>;
}

impl Decodable for u8 {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let mut buffer = [0_u8; 1];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(buffer[0])
    }
}

impl Decodable for u16 {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let mut buffer = [0_u8; 2];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(u16::from_le_bytes(buffer))
    }
}

impl Decodable for u32 {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let mut buffer = [0; 4];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl Decodable for u64 {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let mut buffer = [0; 8];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(u64::from_le_bytes(buffer))
    }
}

impl Decodable for CompactSize {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let n = u8::decode(r)?;

        match n {
            0xFF => {
                let x = u64::decode(r)?;
                Ok(CompactSize(x))
            }
            0xFE => {
                let x = u32::decode(r)?;
                Ok(CompactSize(x as u64))
            }
            0xFD => {
                let x = u16::decode(r)?;
                Ok(CompactSize(x as u64))
            }
            n => Ok(CompactSize(n as u64)),
        }
    }
}

impl Decodable for String {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let len = CompactSize::decode(r)?.0;
        let mut buffer = vec![0; len as usize];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(hex::encode(buffer))
    }
}

impl Decodable for Vec<TxIn> {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let count = CompactSize::decode(r)?.0;
        let mut inputs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            inputs.push(TxIn::decode(r)?);
        }
        Ok(inputs)
    }
}

impl Decodable for TxIn {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        Ok(TxIn {
            tx_id: TxID::decode(r)?,
            output_index: u32::decode(r)?,
            script_sig: String::decode(r)?,
            sequence: u32::decode(r)?,
            witness: Witness::new(),
        })
    }
}

impl Decodable for Witness {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let mut witness_items = vec![];
        let count = u8::decode(r)?;
        for _ in 0..count {
            let len = CompactSize::decode(r)?.0;
            let mut buffer = vec![0; len as usize];
            r.read_exact(&mut buffer).map_err(Error::Io)?;
            witness_items.push(buffer);
        }
        Ok(Witness {
            content: witness_items,
        })
    }
}

impl Decodable for TxID {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let mut buffer = [0_u8; 32];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(TxID(buffer))
    }
}

impl Decodable for Vec<TxOut> {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let count = CompactSize::decode(r)?.0;
        let mut outputs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            outputs.push(TxOut::decode(r)?);
        }
        Ok(outputs)
    }
}

impl Decodable for TxOut {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        Ok(TxOut {
            amount: Amount::from_sat(u64::decode(r)?),
            script_pubkey: String::decode(r)?,
        })
    }
}

impl Decodable for Transaction {
    fn decode<R>(r: &mut R) -> Result<Self, Error>
    where
        R: BufRead + ?Sized,
    {
        let version = u32::decode(r)?;
        let inputs = Vec::<TxIn>::decode(r)?;
        if inputs.is_empty() {
            let segwit_flag = u8::decode(r)?;
            match segwit_flag {
                1 => {
                    let mut inputs = Vec::<TxIn>::decode(r)?;
                    let outputs = Vec::<TxOut>::decode(r)?;
                    for txin in inputs.iter_mut() {
                        txin.witness = Witness::decode(r)?;
                    }
                    if !inputs.is_empty() && inputs.iter().all(|input| input.witness.is_empty()) {
                        Err(Error::ParseFailed(
                            "witness flag set but no witnesses present",
                        ))
                    } else {
                        Ok(Transaction {
                            version,
                            inputs,
                            outputs,
                            lock_time: u32::decode(r)?,
                        })
                    }
                }
                // We don't support anything else
                x => Err(Error::UnsupportedSegwitFlag(x)),
            }
        } else {
            Ok(Transaction {
                version,
                inputs,
                outputs: Vec::<TxOut>::decode(r)?,
                lock_time: u32::decode(r)?,
            })
        }
    }
}

impl Encodable for u8 {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let len = w.write([*self].as_slice()).map_err(Error::Io)?;
        Ok(len)
    }
}

impl Encodable for u16 {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let b = self.to_le_bytes();
        let len = w.write(b.as_slice()).map_err(Error::Io)?;
        Ok(len)
    }
}

impl Encodable for u32 {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let b = self.to_le_bytes();
        let len = w.write(b.as_slice()).map_err(Error::Io)?;
        Ok(len)
    }
}

impl Encodable for u64 {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let b = self.to_le_bytes();
        let len = w.write(b.as_slice()).map_err(Error::Io)?;
        Ok(len)
    }
}

impl Encodable for [u8; 32] {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let len = w.write(self.as_slice()).map_err(Error::Io)?;
        Ok(len)
    }
}

impl Encodable for String {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let b = hex::decode(self).expect("should be a valid hex string");
        let compact_size_len = CompactSize(b.len() as u64).encode(w)?;
        let b_len = w.write(&b).map_err(Error::Io)?;
        Ok(compact_size_len + b_len)
    }
}

impl Encodable for CompactSize {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        match self.0 {
            0..=0xFC => {
                (self.0 as u8).encode(w)?;
                Ok(1)
            }
            0xFD..=0xFFFF => {
                w.write([0xFD].as_slice()).map_err(Error::Io)?;
                (self.0 as u16).encode(w)?;
                Ok(3)
            }
            0x10000..=0xFFFFFFFF => {
                w.write([0xFE].as_slice()).map_err(Error::Io)?;
                (self.0 as u32).encode(w)?;
                Ok(5)
            }
            _ => {
                w.write([0xFF].as_slice()).map_err(Error::Io)?;
                self.0.encode(w)?;
                Ok(9)
            }
        }
    }
}

impl Encodable for Vec<TxIn> {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let mut len = 0;
        len += CompactSize(self.len() as u64).encode(w)?;
        for tx in self.iter() {
            len += tx.encode(w)?;
        }
        Ok(len)
    }
}

impl Encodable for TxID {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        Ok(self.0.encode(w)?)
    }
}

impl Encodable for TxIn {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let mut len = 0;
        len += self.tx_id.encode(w)?;
        len += self.output_index.encode(w)?;
        len += self.script_sig.encode(w)?;
        len += self.sequence.encode(w)?;
        Ok(len)
    }
}

impl Encodable for Vec<TxOut> {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let mut len = 0;
        len += CompactSize(self.len() as u64).encode(w)?;
        for tx in self.iter() {
            len += tx.encode(w)?;
        }
        Ok(len)
    }
}

impl Encodable for Amount {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let len = self.0.encode(w)?;
        Ok(len)
    }
}

impl Encodable for TxOut {
    fn encode<W: Write>(&self, w: &mut W) -> Result<usize, Error> {
        let mut len = 0;
        len += self.amount.encode(w)?;
        len += self.script_pubkey.encode(w)?;
        Ok(len)
    }
}
