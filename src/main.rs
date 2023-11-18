use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod transactions;

fn main() {
    println!("Hello, world!");
    let mut oxsi_coin = Blockchain::new();
    oxsi_coin.add_block(vec![{
        let transaction = transactions::Transaction::new("Alice".to_owned(), "Bob".to_owned(), 10.0);
        // transaction.sign(&transactions::generate_keypair().0);
        // todo fix this
        transaction
    }]);
    println!("{:#?}", oxsi_coin);

}
