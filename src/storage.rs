use crate::models::*;
use serde_json;
use std::fs::*;
use std::io::Write;

static BLOCKCHAIN_FILE: &str = "./files/chain.json";
static TXPOOL_FILE: &str = "./files/txpool.json";

// Saving the Current State of the Blockchain
pub fn save_blockchain_data(blockchain: &Vec<Block>) {
    let mut file = File::create(BLOCKCHAIN_FILE)
        .expect("Error Opening chain.json file in save_blockchain_data");

    let blockchain_list_json = serde_json::to_string_pretty(blockchain).unwrap();

    file.write_all(blockchain_list_json.as_bytes())
        .expect("Error in writing to blockchain.json file");
    file.flush().expect("Error in Flushing");
}

// Retrieving the current state of the Blockchain
pub fn get_blockchain_data() -> Vec<Block> {
    let file =
        File::open(BLOCKCHAIN_FILE).expect("Error Opening chain.json in get_blockchain_data");

    let tmp_blockchain = serde_json::from_reader(&file).unwrap_or(Vec::new());

    return tmp_blockchain;
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
