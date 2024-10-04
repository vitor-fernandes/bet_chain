/*
    bet_chain :)
*/

use std::{thread, time};

use bet_chain::helpers::*;
use bet_chain::miner;
use bet_chain::models::{Block, Transaction};
use bet_chain::storage;

fn main() {
    // Initializing the Blockchain
    let mut blockchain: Vec<Block> = init_blockchain();

    while blockchain.len() <= 5 {
        // Creating a new block with the lastest block information
        let new_block =
            miner::create_new_block(blockchain.last().unwrap(), Vec::<Transaction>::new());
        println!(
            "Mining blocks: Current Block Number: {:?}",
            new_block.number.clone()
        );

        // Adding block to the Blockchain
        blockchain.push(new_block);

        // Commiting to ledger
        storage::save_blockchain_data(&blockchain);

        // Sleeping by 3 secs to mine the next block
        let seconds = time::Duration::from_secs(1);
        thread::sleep(seconds);
    }

    // Just for debug purposes
    //println!("{:#?}", blockchain);
}
