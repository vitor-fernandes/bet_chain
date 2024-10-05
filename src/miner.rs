use crate::{
    models::{Block, Transaction},
    storage,
};

pub fn create_new_block(previous_block: &Block) -> Block {
    // Load the TXs in TX Pool
    let txs: Vec<Transaction> = storage::get_txpool_data();

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

    storage::save_txpool_data(&Vec::<Transaction>::new());

    return block;
}
