use crate::models::*;

use rusty_leveldb::{LdbIterator, DB};

use serde_json;
use std::fs::*;
use std::io::Write;

static TXPOOL_FILE: &str = "./files/txpool.json";

// Saving the Current State of the Blockchain
pub fn save_blockchain_data(blocks: &Vec<Block>) {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open("./files/betchain", opt).unwrap();

    for block in blocks {
        let _ = db.put(
            format!("{:?}", block.number.clone()).as_bytes(),
            block.clone().enconde().as_slice(),
        );
    }
}

pub fn get_blockchain_data() -> Vec<Block> {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open("./files/betchain", opt).unwrap();

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

    return blocks;
}

pub fn get_block_by_number(number: &str) -> Option<Block> {
    let mut opt = rusty_leveldb::Options::default();
    opt.create_if_missing = true;
    let mut db = DB::open("./files/betchain", opt).unwrap();

    let block = db.get(number.as_bytes());

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
