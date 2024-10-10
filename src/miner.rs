use crate::{
    models::{Block, Transaction},
    storage,
};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use rand::Rng;

pub async fn create_new_block(previous_block: &Block) -> Block {
    // Load the TXs in TX Pool
    let txs: Vec<Transaction> = storage::get_txpool_data();
    let mut nonce_rng = rand::thread_rng();

    // Creates a new Block with the Nonce 0
    let mut nonce: u64 = 0;
    let mut block: Block = Block::new(
        previous_block.hash.clone(),
        previous_block.number.clone(),
        nonce,
        txs.clone(),
    );

    let last_block: &Block = previous_block;

    // Simple PoW with sequential nonce update + block difficulty 4
    while !block.hash.starts_with("0".repeat(4).as_str()) {
        nonce = nonce_rng.gen::<u64>();
        block = Block::new(
            last_block.hash.clone(),
            last_block.number.clone(),
            nonce,
            txs.clone(),
        );
    }

    // Cleaning the TXPool
    storage::save_txpool_data(&Vec::<Transaction>::new());

    // Storing the new block into the ledger
    storage::save_blockchain_data(&block);

    // Creating the Stream and sending the new mined block to the P2P Implementation
    let mut stream = TcpStream::connect("127.0.0.1:55666").await.unwrap();
    stream
        .write(
            format!(
                "forward_block|{:?}",
                serde_json::to_string(&block.clone()).unwrap()
            )
            .as_bytes(),
        )
        .await
        .unwrap();

    return block;
}
