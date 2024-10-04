use crate::models::{Block, Transaction};

pub fn create_new_block(previous_block: &Block, txs: Vec<Transaction>) -> Block {
    // Creates a new Block with the Nonce 0
    let mut nonce: u64 = 0;
    let mut block: Block = Block::new(
        previous_block.hash.clone(),
        previous_block.number.clone(),
        nonce,
        txs.clone(),
    );

    // Simple PoW with sequential nonce update + block difficulty 2
    while !block.hash.starts_with("00") {
        nonce += 1;
        block = Block::new(
            previous_block.hash.clone(),
            previous_block.number.clone(),
            nonce,
            txs.clone(),
        );
    }

    return block;
}

pub fn create_new_tx(from: String, to: String, amount: u64) -> Transaction {
    let tx = Transaction::new(from, to, amount);
    return tx;
}
