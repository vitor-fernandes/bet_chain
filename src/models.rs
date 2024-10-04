use crate::helpers;
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub number: u64,
    pub nonce: u64,
    pub tx_root: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        previous_hash: String,
        previous_number: u64,
        nonce: u64,
        transactions: Vec<Transaction>,
    ) -> Block {
        let block_number: u64 = previous_number.clone() + 1;

        let tx_root = helpers::gen_tx_root(&transactions);

        let dig: String = format!(
            "{}{}{}{}",
            previous_hash.clone(),
            nonce,
            block_number,
            tx_root
        );

        let hash = sha256::digest(&dig);

        let number = previous_number + 1;

        let tmp_block = Block {
            hash,
            previous_hash,
            nonce,
            number,
            tx_root,
            transactions,
        };

        return tmp_block;
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    hash: String,
    from_address: String,
    to_address: String,
    amount: u64,
    timestamp: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64) -> Transaction {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hash: String = digest(format!(
            "from:{},to:{},amount:{},timestamp:{}",
            from, to, amount, timestamp
        ));

        let tx = Transaction {
            from_address: from,
            to_address: to,
            amount,
            hash,
            timestamp,
        };

        return tx;
    }

    pub fn to_string(&self) -> String {
        return format!(
            "from:{},to:{},amount:{},hash:{},timestamp:{}",
            self.from_address, self.to_address, self.amount, self.hash, self.timestamp
        );
    }
}
