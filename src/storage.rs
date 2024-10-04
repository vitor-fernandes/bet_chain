use crate::models::*;
use serde_json;
use std::fs::*;
use std::io::Write;

static BLOCKCHAIN_FILE: &str = "./files/blockchain.json";

// Saving the Current State of the Blockchain
pub fn save_blockchain_data(blockchain: &Vec<Block>) {
    let mut file = File::create(BLOCKCHAIN_FILE)
        .expect("Error Opening the blockchain.json file in save_blockchain_data");

    let blockchain_list_json = serde_json::to_string_pretty(blockchain).unwrap();

    file.write_all(blockchain_list_json.as_bytes())
        .expect("Error in writing to blockchain.json file");
    file.flush().expect("Error in Flushing");
}

// Retrieving the current state of the Blockchain
pub fn get_blockchain_data() -> Vec<Block> {
    let file = File::open(BLOCKCHAIN_FILE)
        .expect("Error Opening the blockchain.json in get_blockchain_data");

    let tmp_blockchain = serde_json::from_reader(&file).unwrap_or(Vec::new());

    return tmp_blockchain;
}
