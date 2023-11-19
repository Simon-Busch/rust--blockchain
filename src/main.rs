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
        transaction.sign(&alice_wallet.secret_key);
        let ok_transac = transaction.verify_transaction(&transaction, &transaction.signature.unwrap(), &alice_wallet.public_key);
        println!("Transaction verified: {}", ok_transac);
        transaction
    }]);
    for _ in 0..10 {
        oxsi_coin.add_block(vec![{
            let alice_wallet = wallet::Wallet::new();
            let bob_wallet = wallet::Wallet::new();
            let mut transaction = transactions::Transaction::new(alice_wallet, bob_wallet, 10.0);
            transaction.sign(&alice_wallet.secret_key);
            let ok_transac = transaction.verify_transaction(&transaction, &transaction.signature.unwrap(), &alice_wallet.public_key);
            println!("Transaction verified: {}", ok_transac);
            transaction
        }]);
    }
    println!("{:#?}", oxsi_coin);

}
