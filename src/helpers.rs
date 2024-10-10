use crate::{
    models::{Block, Transaction},
    storage,
};

pub fn init_blockchain() {
    storage::save_balance_of("betty".to_string(), 666666);

    let blocks: Vec<Block> = storage::get_blockchain_data();

    // Genesis Block generation and insertion
    if blocks.len() == 0 {
        let transactions: Vec<Transaction> = Vec::new();
        let tx_root = gen_tx_root(&transactions);

        let genesis_block: Block = Block {
            hash: "0".repeat(64).to_string(),
            previous_hash: "null".to_string(),
            number: 0,
            nonce: 0,
            tx_root,
            transactions,
        };

        // Adding the Genesis Block to the Ledger
        storage::save_blockchain_data(&genesis_block);
    }
}

pub fn gen_tx_root(transactions: &Vec<Transaction>) -> String {
    // Creating a string containing all tx separated by the char |
    let tx_to_string: String = transactions
        .iter()
        .map(|x| format!("{}|", x.to_string()))
        .collect();

    // Creating the sha256 of all txs
    let tx_root = sha256::digest(tx_to_string);

    return tx_root;
}
