use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod transactions;
mod wallet;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let mut oxsi_coin = Blockchain::new();
    let alice_wallet = wallet::Wallet::new();
    let bob_wallet = wallet::Wallet::new();
    oxsi_coin.add_block(vec![{
        let mut transaction =
            transactions::Transaction::new(alice_wallet, bob_wallet, 10.0, Some("test message"));
        transaction.sign(&alice_wallet.secret_key);
        let ok_transac = transaction.verify_transaction(
            &transaction,
            &transaction.signature.unwrap(),
            &alice_wallet.public_key,
        );
        println!("Transaction verified: {}", ok_transac);
        transaction
    }]);

    for _ in 0..10 {
        let alice_wallet = wallet::Wallet::new();
        let bob_wallet = wallet::Wallet::new();
        let amount: f32 = rng.gen_range(1.0..100.0);
        let message = Some(format!("I sent you: {}", amount));

        let mut transaction = transactions::Transaction::new(
            alice_wallet,
            bob_wallet,
            amount,
            message.as_deref(),
        );
        transaction.sign(&alice_wallet.secret_key);
        let ok_transac = transaction.verify_transaction(
            &transaction,
            &transaction.signature.unwrap(),
            &alice_wallet.public_key,
        );
        println!("Transaction verified: {}", ok_transac);

        if let Some(decoded_message) = transaction.decode_message() {
            println!("Decoded Message: {}", decoded_message);
        } else {
            println!("Failed to decode message.");
        }

        oxsi_coin.add_block(vec![transaction]);
    }
    println!("{:#?}", oxsi_coin);
}
