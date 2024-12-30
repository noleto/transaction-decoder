use crate::encode_decode::Encodable;
use crate::types::{Transaction, TxID};
use sha2::{Digest, Sha256};

impl Transaction {
    pub fn compute_txid(&self) -> TxID {
        let mut txid_data = Vec::new();
        self.version
            .encode(&mut txid_data)
            .expect("writing to a vec shouldn't fail");
        self.inputs
            .encode(&mut txid_data)
            .expect("writing to a vec shouldn't fail");
        self.outputs
            .encode(&mut txid_data)
            .expect("writing to a vec shouldn't fail");
        self.lock_time
            .encode(&mut txid_data)
            .expect("writing to a vec shouldn't fail");
        TxID::from_raw_transaction(txid_data)
    }
}

impl TxID {
    pub fn from_hash(bytes: [u8; 32]) -> Self {
        TxID(bytes)
    }

    fn from_raw_transaction(tx: Vec<u8>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&tx);
        let hash1 = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(hash1);
        let hash2 = hasher.finalize();

        TxID::from_hash(hash2.into())
    }
}
