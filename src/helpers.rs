use crate::{
    models::{Blockchain, Transaction},
    storage,
};

pub fn init_blockchain() -> Blockchain {
    storage::save_balance_of("betty".to_string(), 666666);
    return Blockchain::new();
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
