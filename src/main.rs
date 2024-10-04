/*
    bet_chain :)
*/

use bet_chain::helpers;
use bet_chain::miner;
use bet_chain::models::{Block, Blockchain};
use bet_chain::rpc;
use std::sync::mpsc;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let (main_tx, main_rx) = mpsc::channel();

    // Initializing the Blockchain
    let mut blockchain: Blockchain = helpers::init_blockchain();

    thread::spawn(move || {
        rpc::start(main_tx);
    });

    // Running Forever
    loop {
        let tx_watcher = main_rx.try_recv();

        match tx_watcher {
            Ok(tx) => {
                blockchain.tx_pool.add_new(tx.unwrap());
            }
            _ => {}
        }

        // TMP impl
        let tmp_blockchain = blockchain.clone();
        let last_block: &Block = tmp_blockchain.get_last_block();
        // // // // // //

        // Creating a new block with the lastest block information
        let new_block = miner::create_new_block(last_block, blockchain.get_current_txs());

        // Logging current block
        println!(
            "Mining blocks: Current Block Number: {:?}",
            new_block.number.clone()
        );

        // Saving the new block into the storage
        blockchain.insert_block(new_block);

        // Sleeping by 3 secs to mine the next block
        let seconds = time::Duration::from_secs(3);
        thread::sleep(seconds);
    }
}
