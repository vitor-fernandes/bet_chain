/*
    bet_chain :)
*/

use bet_chain::helpers;
use bet_chain::miner;
use bet_chain::models::{Block, Blockchain, TXPool};
use bet_chain::p2p;
use bet_chain::storage;
use std::{thread, time};

#[tokio::main]
async fn main() {
    // Initializing the Blockchain
    let mut blockchain: Blockchain = helpers::init_blockchain();

    // Possible Peers to connect
    let peers: Vec<String> = ["localhost:55666".to_string()].to_vec();

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
        // Sleeping by 3 secs to mine the next block
        let seconds = time::Duration::from_secs(3);
        thread::sleep(seconds);

        // TMP impl
        let tmp_blockchain = blockchain.clone();
        let last_block: &Block = tmp_blockchain.get_last_block();
        // // // // // //

        // Creating a new block with the lastest block information
        let new_block = miner::create_new_block(last_block).await;

        // Logging current block
        println!(
            "Mining blocks: Current Block Number: {:?}",
            new_block.clone().number
        );

        // Saving the new block into the storage
        blockchain.insert_block(new_block);
    }
}
