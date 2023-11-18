use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use sha2::{Sha256, Digest};
use std::str::FromStr;
use crate::wallet::Wallet;
use std::fmt::Write;

#[derive(Debug)]
pub struct Transaction {
    pub sender: Wallet,
    pub recipient: Wallet,
    pub amount: f32,
    pub signature: String,
}

impl Transaction {

    pub fn new(sender: Wallet, recipient: Wallet, amount: f32) -> Self {
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

      // Ensure the message is properly formatted or serialized before signing
      let mut hasher = Sha256::new();
      hasher.update(&message);
      let hash = hasher.finalize();

      // Sign the hash of the message
      let message_to_sign = Message::from_slice(&hash).expect("Invalid message");
      let signature = context.sign(&message_to_sign, secret_key);
      let signature_bytes = signature.serialize_compact().to_vec();
      let mut signature_str = String::new();
      for byte in signature_bytes {
          write!(signature_str, "{:02x}", byte).expect("Failed to write");
      }
      // self.verify(signature_str);
      self.signature = signature_str;
  }

    pub fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.sender.public_key.serialize());
        bytes.extend(self.recipient.public_key.serialize());
        bytes.extend(&self.amount.to_le_bytes());
        bytes
    }

    // Verify that the signature is correct
    pub fn verify(&self, public_key: &PublicKey) -> bool {
      let context = Secp256k1::new();
      let message = self.create_message();

      // Convert the hexadecimal signature string to bytes
      let mut signature_bytes = Vec::new();
      for i in (0..self.signature.len()).step_by(2) {
          let byte = u8::from_str_radix(&self.signature[i..i + 2], 16)
              .expect("Failed to parse byte");
          signature_bytes.push(byte);
      }

      let signature = Signature::from_compact(&signature_bytes).expect("Invalid signature");

      context
          .verify(
              &Message::from_slice(&message).expect("Invalid message"),
              &signature,
              public_key,
          )
          .is_ok()
  }
}
