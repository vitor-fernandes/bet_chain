use crate::models::*;

use rocksdb::{IteratorMode, Options, DB};

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
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, BLOCKS_FILE).unwrap();

    let _ = db.put(
        format!("{:?}", block.number.clone()).as_bytes(),
        block.clone().enconde().as_slice(),
    );

    let _ = db.put("latest", block.clone().enconde().as_slice());

    let _ = DB::destroy(&opt, BLOCKS_FILE);
}

pub fn get_blockchain_data() -> Vec<Block> {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, BLOCKS_FILE).unwrap();

    let mut blocks: Vec<Block> = Vec::<Block>::new();

    let iter_blocks = db.iterator(IteratorMode::Start);

    for iter in iter_blocks {
        let (_, value) = iter.unwrap();
        let block: Block = serde_json::from_slice(&value).unwrap();
        blocks.push(block);
    }

    blocks.sort_by_key(|block| block.number);

    let _ = DB::destroy(&opt, BLOCKS_FILE);

    return blocks;
}

pub fn get_block_by_number(number: &str) -> Option<Block> {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, BLOCKS_FILE).unwrap();

    let block = db.get(number.as_bytes());
    let _ = DB::destroy(&opt, BLOCKS_FILE);

    match block {
        Ok(data) => match data {
            Some(tmp_block) => {
                let block_obj: Block = serde_json::from_slice(tmp_block.as_slice()).unwrap();
                return Some(block_obj);
            }
            None => None,
        },
        Err(e) => {
            print!("Error getting block number: {e:?}");
            None
        }
    }
}

pub fn get_last_mined_block() -> Option<Block> {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, BLOCKS_FILE).unwrap();

    let block = db.get("latest");
    let _ = DB::destroy(&opt, BLOCKS_FILE);

    match block {
        Ok(data) => match data {
            Some(tmp_block) => {
                let block_obj: Block = serde_json::from_slice(tmp_block.as_slice()).unwrap();
                return Some(block_obj);
            }
            None => None,
        },
        Err(e) => {
            print!("Error getting block number: {e:?}");
            None
        }
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
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, BALANCES_FILE).unwrap();

    let block = db.get(user.as_bytes());

    let _ = DB::destroy(&opt, BALANCES_FILE);

    match block {
        Ok(data) => match data {
            Some(value) => {
                let balance: u64 = serde_json::from_slice(value.as_slice()).unwrap();
                return balance;
            }
            None => 0,
        },
        Err(_) => {
            let balance_zero_u64: u64 = 0;
            save_balance_of(user, balance_zero_u64);
            let _ = return balance_zero_u64;
        }
    }
}

pub fn save_balance_of(user: String, balance: u64) {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, BALANCES_FILE).unwrap();

    let _ = db.put(user.as_bytes(), balance.to_string().as_bytes());
    let _ = DB::destroy(&opt, BALANCES_FILE);
}

pub fn save_transaction(tx: Transaction) {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, TRANSACTIONS_FILE).unwrap();

    let _ = db.put(tx.hash.clone().as_bytes(), tx.enconde().as_slice());
    let _ = DB::destroy(&opt, TRANSACTIONS_FILE);
}

pub fn get_transaction(hash: String) -> Option<Transaction> {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, TRANSACTIONS_FILE).unwrap();

    match db.get(hash.as_bytes()) {
        Ok(data) => match data {
            Some(value) => {
                let tx: Transaction = serde_json::from_slice(&value).unwrap();
                let _ = DB::destroy(&opt, TRANSACTIONS_FILE);
                return Some(tx);
            }
            None => None,
        },
        Err(_) => {
            let _ = DB::destroy(&opt, TRANSACTIONS_FILE);
            return None;
        }
    }
}

pub fn save_transaction_of_user(user: String, txs: Vec<String>) {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, TRANSACTIONS_FILE).unwrap();

    let all_txs: String = txs.join(",");

    let _ = db.put(user.as_bytes(), all_txs.as_bytes());
    let _ = DB::destroy(&opt, TRANSACTIONS_FILE);
}

pub fn get_transactions_of_user(user: String) -> Option<Vec<String>> {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, TRANSACTIONS_FILE).unwrap();

    let txs = db.get(user.as_bytes());

    match txs {
        Ok(data) => match data {
            Some(value) => {
                let mut tmp = String::from("");

                for c in value.iter() {
                    tmp.push(char::from(c.to_owned()));
                }

                let txs: Vec<&str> = tmp.split(",").collect::<Vec<&str>>();

                let mut tmp: Vec<String> = Vec::new();
                for tx in txs.iter() {
                    tmp.push(tx.to_string());
                }

                let _ = DB::destroy(&opt, TRANSACTIONS_FILE);
                return Some(tmp);
            }
            None => Some(Vec::new()),
        },
        Err(_) => {
            let _ = DB::destroy(&opt, TRANSACTIONS_FILE);
            return Some(Vec::new());
        }
    }
}

pub fn get_user_nonce(user: String) -> u64 {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, NONCES_FILE).unwrap();

    match db.get(user.as_bytes()) {
        Ok(data) => match data {
            Some(value) => {
                let nonce: u64 = serde_json::from_slice(&value).unwrap();
                let _ = DB::destroy(&opt, NONCES_FILE);
                return nonce;
            }
            None => 0,
        },
        Err(_) => {
            let _ = DB::destroy(&opt, NONCES_FILE);
            return 0;
        }
    }
}

pub fn set_user_nonce(user: String, nonce: u64) {
    let mut opt = Options::default();
    opt.create_if_missing(true);
    let db = DB::open(&opt, NONCES_FILE).unwrap();

    let _ = db.put(user.as_bytes(), nonce.to_string().as_bytes());
    let _ = DB::destroy(&opt, NONCES_FILE);
}
