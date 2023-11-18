use crate::transactions::Transaction;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub proof: u64,
}

// Implemntation of the block
impl Block {
    pub fn new(
        timestamp: i64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        proof: u64,
    ) -> Self {
        let mut block = Self {
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            proof,
        };
        block.hash = Block::calculate_hash(&block);
        block
    }

    fn calculate_hash(block: &Block) -> String {
        let mut hasher = Sha256::new();
        hasher.update(block.timestamp.to_string().as_bytes());
        hasher.update(&block.transactions.len().to_string().as_bytes()); //Todo check this
        hasher.update(&block.previous_hash.as_bytes());
        let hash = hasher.finalize();
        let mut hash_str = String::new();
        for byte in hash {
            write!(&mut hash_str, "{:02x}", byte).expect("Unable to write");
        }
        hash_str
    }

    pub fn mine_block(last_proof: u64) -> u64 {
      let mut proof = 0;
      while !Self::valid_proof(last_proof, proof) {
          proof += 1;
      }
      proof
  }

    pub fn valid_proof(last_proof: u64, proof: u64) -> bool {
      let guess = format!("{}{}", last_proof, proof);
      let guess_bytes = guess.as_bytes();

      let mut hasher = Sha256::new();
      hasher.update(guess_bytes);
      let hash_result = hasher.finalize();

      let hash_string = format!("{:x}", hash_result);

      hash_string.ends_with("0000")
  }
}
