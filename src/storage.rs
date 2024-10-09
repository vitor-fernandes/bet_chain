use crate::models::*;

use rusty_leveldb::{LdbIterator, DB};

use serde_json;
use std::fs::*;
use std::io::Write;

static BLOCKS_FILE: &str = "./files/betchain/blocks";
static BALANCES_FILE: &str = "./files/betchain/balances";
static TRANSACTIONS_FILE: &str = "./files/betchain/transactions";
static NONCES_FILE: &str = "./files/betchain/nonces";

static TXPOOL_FILE: &str = "./files/txpool.json";

// Saving the Current State of the Blockchain
pub fn save_blockchain_data(block: &Block) {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(BLOCKS_FILE, opt).unwrap();

    let _ = db.put(
        format!("{:?}", block.number.clone()).as_bytes(),
        block.clone().enconde().as_slice(),
    );

    let _ = db.close();
}

pub fn get_blockchain_data() -> Vec<Block> {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(BLOCKS_FILE, opt).unwrap();

    let mut blocks: Vec<Block> = Vec::<Block>::new();

    let mut current_blocks = db.new_iter().unwrap();

    loop {
        let tmp_block = current_blocks.next();
        match tmp_block {
            Some(data) => {
                let block: Block = serde_json::from_slice(data.1.as_slice()).unwrap();
                blocks.push(block);
            }
            None => {
                break;
            }
        }
    }

    blocks.sort_by_key(|block| block.number);

    let _ = db.close();

    return blocks;
}

pub fn get_block_by_number(number: &str) -> Option<Block> {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(BLOCKS_FILE, opt).unwrap();

    let block = db.get(number.as_bytes());

    let _ = db.close();

    match block {
        Some(data) => {
            let tmp_block: Block = serde_json::from_slice(data.as_slice()).unwrap();
            return Some(tmp_block);
        }
        None => None,
    }
}

pub fn save_txpool_data(tx: &Vec<Transaction>) {
    let mut file =
        File::create(TXPOOL_FILE).expect("Error Opening txpool.json file in save_blockchain_data");

    let txpool_list_json = serde_json::to_string_pretty(tx).unwrap();

    file.write_all(txpool_list_json.as_bytes())
        .expect("Error in writing to blockchain.json file");
    file.flush().expect("Error in Flushing");
}

pub fn get_txpool_data() -> Vec<Transaction> {
    let file = File::open(TXPOOL_FILE).expect("Error Opening txpool.json in get_blockchain_data");

    let tmp_txpool = serde_json::from_reader(&file).unwrap_or(Vec::new());

    return tmp_txpool;
}

pub fn get_balance_of(user: String) -> u64 {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(BALANCES_FILE, opt).unwrap();

    let block = db.get(user.as_bytes());

    let _ = db.close();

    match block {
        Some(data) => {
            let balance: u64 = serde_json::from_slice(data.as_slice()).unwrap();
            return balance;
        }
        None => {
            let balance_zero_u64: u64 = 0;
            save_balance_of(user, balance_zero_u64);
            let _ = return balance_zero_u64;
        }
    }
}

pub fn save_balance_of(user: String, balance: u64) {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(BALANCES_FILE, opt).unwrap();

    let _ = db.put(user.as_bytes(), balance.to_string().as_bytes());
    let _ = db.close();
}

pub fn save_transaction(tx: Transaction) {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(TRANSACTIONS_FILE, opt).unwrap();

    let _ = db.put(tx.hash.clone().as_bytes(), tx.enconde().as_slice());
    let _ = db.close();
}

pub fn get_transaction(hash: String) -> Option<Transaction> {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(TRANSACTIONS_FILE, opt).unwrap();

    match db.get(hash.as_bytes()) {
        Some(data) => {
            let tx: Transaction = serde_json::from_slice(&data).unwrap();
            let _ = db.close();
            return Some(tx);
        }
        None => {
            let _ = db.close();
            return None;
        }
    };
}

pub fn save_transaction_of_user(user: String, txs: Vec<String>) {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(TRANSACTIONS_FILE, opt).unwrap();

    let all_txs: String = txs.join(",");

    let _ = db.put(user.as_bytes(), all_txs.as_bytes());
    let _ = db.close();
}

pub fn get_transactions_of_user(user: String) -> Option<Vec<String>> {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(TRANSACTIONS_FILE, opt).unwrap();

    match db.get(user.as_bytes()) {
        Some(data) => {
            let mut tmp = String::from("");

            for c in data.iter() {
                tmp.push(char::from(c.to_owned()));
            }

            let txs: Vec<&str> = tmp.split(",").collect::<Vec<&str>>();

            let mut tmp: Vec<String> = Vec::new();
            for tx in txs.iter() {
                tmp.push(tx.to_string());
            }

            let _ = db.close();
            return Some(tmp);
        }
        None => {
            let _ = db.close();
            return Some(Vec::new());
        }
    };
}

pub fn get_user_nonce(user: String) -> u64 {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(NONCES_FILE, opt).unwrap();

    match db.get(user.as_bytes()) {
        Some(data) => {
            let nonce: u64 = serde_json::from_slice(&data).unwrap();
            let _ = db.close();
            return nonce;
        }
        None => {
            let _ = db.close();
            return 0;
        }
    };
}

pub fn set_user_nonce(user: String, nonce: u64) {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open(NONCES_FILE, opt).unwrap();

    let _ = db.put(user.as_bytes(), nonce.to_string().as_bytes());
    let _ = db.close();
}
