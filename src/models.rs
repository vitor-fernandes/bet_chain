use crate::helpers;
use crate::rpc;
use crate::storage;
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

        // Generating the tx_root of block's transactions
        let tx_root = helpers::gen_tx_root(&transactions);

        // Digest to create the Hash of current Block
        let dig: String = format!(
            "{}{}{}{}",
            previous_hash.clone(),
            nonce,
            block_number,
            tx_root
        );

        // Calculating the Hash of the Block
        let hash = sha256::digest(&dig);

        let tmp_block = Block {
            hash,
            previous_hash,
            nonce,
            number: block_number,
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

    // Helper to print and encode tx
    pub fn to_string(&self) -> String {
        return format!(
            "from:{},to:{},amount:{},hash:{},timestamp:{}",
            self.from_address, self.to_address, self.amount, self.hash, self.timestamp
        );
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blocks: Vec<Block> = storage::get_blockchain_data();

        // Genesis Block generation and insertion
        if blocks.len() == 0 {
            let transactions: Vec<Transaction> = Vec::new();
            let tx_root = helpers::gen_tx_root(&transactions);

            let genesis_block: Block = Block {
                hash: "0".repeat(64).to_string(),
                previous_hash: "null".to_string(),
                number: 0,
                nonce: 0,
                tx_root,
                transactions,
            };

            // Adding the Genesis Block to the Ledger
            blocks.push(genesis_block);

            // Save current blockchain's state
            storage::save_blockchain_data(&blocks);
        }

        return Blockchain { blocks };
    }

    pub fn insert_block(&mut self, block: Block) {
        self.blocks.push(block);
        storage::save_blockchain_data(&self.blocks);
    }

    pub fn get_last_block(&self) -> &Block {
        return self.blocks.last().unwrap();
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXPool {
    transactions: Vec<Transaction>,
}

impl TXPool {
    pub fn new() -> TXPool {
        return TXPool {
            transactions: Vec::<Transaction>::new(),
        };
    }

    pub fn start(&mut self) {
        let (txpool_tx, txpool_rx) = mpsc::channel();

        thread::spawn(move || {
            rpc::start(txpool_tx);
        });

        loop {
            let tx_watcher = txpool_rx.try_recv();

            match tx_watcher {
                Ok(tx) => {
                    self.insert_tx_into_pool(tx.unwrap());
                }
                _ => {}
            }
        }
    }

    fn insert_tx_into_pool(&mut self, tx: Transaction) {
        self.transactions.push(tx);
        storage::save_txpool_data(&self.transactions);
    }
}
