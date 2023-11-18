use crate::blockchain::Blockchain;

mod block;
mod blockchain;

fn main() {
    println!("Hello, world!");
    let mut oxsi_coin = Blockchain::new();
    oxsi_coin.add_block("Block 1 Data".to_owned());
    oxsi_coin.add_block("Block 2 Data".to_owned());
    println!("{:#?}", oxsi_coin);

}
