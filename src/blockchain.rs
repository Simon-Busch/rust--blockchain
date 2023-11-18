use crate::block::Block;


#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}
// Implementing the Blockchain
impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_owned(), String::new());
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(Self::current_timestamp(), data, previous_hash);
        self.chain.push(new_block);
    }
    pub fn current_timestamp() -> i64 {
        //   placeholder value
        1_617_439_785
    }
}
