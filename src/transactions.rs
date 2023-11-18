use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use std::str::FromStr;

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f32,
    pub signature: String,
}

impl Transaction {

    pub fn new(sender: String, recipient: String, amount: f32) -> Self {
        Self {
            sender,
            recipient,
            amount,
            signature: String::new(),
        }
    }

    // Assume `self` has the sender, recipient, and amount fields populated.
    pub fn sign(&mut self, secret_key: &SecretKey) {
        let context = Secp256k1::new();
        let message = self.create_message();
        let signature = context.sign(&Message::from_slice(&message).unwrap(), secret_key);

        self.signature = signature.to_string();
    }

    pub fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.recipient.as_bytes());
        bytes.extend(&self.amount.to_le_bytes());
        bytes
    }

    // Verify that the signature is correct
    pub fn verify(&self, public_key: &PublicKey) -> bool {
        let context = Secp256k1::new();
        let message = self.create_message();
        let signature = Signature::from_str(&self.signature).unwrap();

        context
            .verify(
                &Message::from_slice(&message).unwrap(),
                &signature,
                public_key,
            )
            .is_ok()
    }
}
