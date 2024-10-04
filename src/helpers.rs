use crate::models::*;
use crate::storage::*;

pub fn init_blockchain() -> Vec<Block> {
    let mut tmp_blockchain = get_blockchain_data();

    // Checking if the current ledger was already started
    if tmp_blockchain.len() == 0 {
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
        tmp_blockchain.push(genesis_block);

        save_blockchain_data(&tmp_blockchain);
    }

    return tmp_blockchain;
}

pub fn gen_tx_root(transactions: &Vec<Transaction>) -> String {
    let tx_to_string: String = transactions
        .iter()
        .map(|x| format!("{}|", x.to_string()))
        .collect();

    let tx_root = sha256::digest(tx_to_string);

    return tx_root;
}
