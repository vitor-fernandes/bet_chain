use crate::models::{Block, Transaction};

pub fn create_new_block(previous_block: &Block, txs: Vec<Transaction>) -> Block {
    let mut nonce: u64 = 0;
    let mut block: Block = Block::new(
        previous_block.hash.clone(),
        previous_block.number.clone(),
        nonce,
        txs.clone(),
    );
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
