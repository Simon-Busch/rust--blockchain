use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod transactions;
mod wallet;

fn main() {
    let mut oxsi_coin = Blockchain::new();
    let alice_wallet = wallet::Wallet::new();
    let bob_wallet = wallet::Wallet::new();
    oxsi_coin.add_block(vec![{
        let mut transaction = transactions::Transaction::new(alice_wallet, bob_wallet, 10.0);
        // transaction.sign(&alice_wallet.secret_key);
        transaction
    }]);
    // // Create a loop to add 10 blocks with random transaction and wait for 10 second between each loop
    for _ in 0..10 {
        oxsi_coin.add_block(vec![{
            let alice_wallet = wallet::Wallet::new();
            let bob_wallet = wallet::Wallet::new();
            let transaction = transactions::Transaction::new(alice_wallet, bob_wallet, 10.0);
            // transaction.sign(&transactions::generate_keypair().0);
            // todo fix this
            transaction
        }]);
    }
    println!("{:#?}", oxsi_coin);

}
