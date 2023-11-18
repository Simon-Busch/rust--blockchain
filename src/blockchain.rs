use std::time::{SystemTime, UNIX_EPOCH};

use crate::{block::Block, transactions::Transaction};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

// Implementing the Blockchain
impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(Self::current_timestamp(), vec![], String::new(), 0);
        let chain = vec![genesis_block];
        Self { chain }
    }

    pub fn add_block(&mut self, data: Vec<Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let previous_proof = self.chain.last().unwrap().proof.clone();
        let current_proof = Block::mine_block(previous_proof);
        let new_block = Block::new(
            Self::current_timestamp(),
            data,
            previous_hash,
            current_proof,
        );
        self.chain.push(new_block);
    }

    pub fn current_timestamp() -> i64 {
        //   placeholder value
        // get current timestamp
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}
