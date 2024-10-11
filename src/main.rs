/*
    bet_chain :)
*/

use bet_chain::helpers;
use bet_chain::miner;
use bet_chain::models::{Block, TXPool};
use bet_chain::p2p;
use bet_chain::storage;
use std::thread;

#[tokio::main]
async fn main() {
    // Initializing the Blockchain
    helpers::init_blockchain();

    // Possible Peers to connect
    let peers: Vec<String> = ["localhost:55667".to_string()].to_vec();

    // Spawning the P2P Server
    thread::spawn(move || {
        p2p::start(peers);
    });

    // Pool containing all TXs
    let mut txpool: TXPool = TXPool::new();

    // Spawning the TX Pool worker
    thread::spawn(move || {
        txpool.start();
    });

    // Running Forever
    loop {
        // Last mined block knowed by this Node
        let last_block: Block = storage::get_last_mined_block().unwrap();

        // Creating a new block with the lastest block information
        let new_block = miner::create_new_block(&last_block).await;

        // Logging current block
        println!(
            "Mining blocks: Current Block Number: {:?}",
            new_block.clone().number
        );
    }
}
