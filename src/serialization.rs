use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use crate::types::{Transaction, TxID, TxIn, TxOut, Witness};

impl Serialize for Transaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tx = serializer.serialize_struct("Transaction", 5)?;
        tx.serialize_field("transaction_id", &self.compute_txid())?;
        tx.serialize_field("version", &self.version)?;
        tx.serialize_field("inputs", &self.inputs)?;
        tx.serialize_field("outputs", &self.outputs)?;
        tx.serialize_field("lock_time", &self.lock_time)?;
        tx.end()
    }
}

impl Serialize for TxID {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut bytes = self.0.clone();
        bytes.reverse();
        s.serialize_str(&hex::encode(bytes))
    }
}

impl Serialize for TxIn {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut txin = s.serialize_struct("TxIn", 5)?;
        txin.serialize_field("txid", &self.tx_id)?;
        txin.serialize_field("vout", &self.output_index)?;
        txin.serialize_field("script_sig", &self.script_sig)?;

        if !&self.witness.is_empty() {
            txin.serialize_field("txinwitness", &self.witness)?;
        }

        txin.serialize_field("sequence", &self.sequence)?;
        txin.end()
    }
}

impl Serialize for TxOut {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut txout = s.serialize_struct("TxOut", 2)?;
        txout.serialize_field("amount", &self.amount.to_btc())?;
        txout.serialize_field("script_pubkey", &self.script_pubkey)?;
        txout.end()
    }
}

impl Serialize for Witness {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = s.serialize_seq(Some(self.content.len()))?;

        for elem in self.content.iter() {
            seq.serialize_element(&hex::encode(&elem))?;
        }

        seq.end()
    }
}
