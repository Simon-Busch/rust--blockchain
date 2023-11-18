use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod transactions;

fn main() {
    println!("Hello, world!");
    let mut oxsi_coin = Blockchain::new();
    oxsi_coin.add_block("Block 1 Data".to_owned());
    oxsi_coin.add_block("Block 2 Data".to_owned());
    oxsi_coin.add_block("Block 3 Data".to_owned());
    oxsi_coin.add_block("Block 4 Data".to_owned());
    oxsi_coin.add_block("Block 5 Data".to_owned());
    oxsi_coin.add_block("Block 6 Data".to_owned());
    println!("{:#?}", oxsi_coin);

}
