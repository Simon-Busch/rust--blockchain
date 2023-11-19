use crate::wallet::Wallet;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Transaction {
  pub sender: Wallet,
  pub recipient: Wallet,
  pub amount: f32,
  pub signature: Option<Signature>,
  pub message: Option<Vec<u8>>,
}

impl Transaction {
  pub fn new(sender: Wallet, recipient: Wallet, amount: f32, message: Option<&str>) -> Self {
      let message_bytes = message.map(|msg| msg.as_bytes().to_vec());
      Self {
          sender,
          recipient,
          amount,
          signature: None,
          message: message_bytes,
      }
  }

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

        // Store the signature in the transaction
        self.signature = Some(signature);
    }

    pub fn verify_transaction(
        &self,
        transaction: &Transaction,
        signature: &Signature,
        public_key: &PublicKey,
    ) -> bool {
        let secp = Secp256k1::new();
        let message = Message::from_slice(&Self::hash_transaction(transaction)).expect("32 bytes");

        secp.verify(&message, signature, public_key).is_ok()
    }

    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.sender.public_key.serialize());
        bytes.extend(self.recipient.public_key.serialize());
        bytes.extend(&self.amount.to_le_bytes());
        bytes
    }

    fn hash_transaction(transaction: &Transaction) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // Serialize the sender's and recipient's public keys and amount to create the message
        hasher.update(&transaction.sender.public_key.serialize());
        hasher.update(&transaction.recipient.public_key.serialize());
        hasher.update(&transaction.amount.to_le_bytes());

        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    pub fn decode_message(&self) -> Option<String> {
      if let Some(msg_bytes) = &self.message {
          // Attempt to convert the message bytes into a UTF-8 string
          if let Ok(msg_str) = std::str::from_utf8(msg_bytes) {
              return Some(msg_str.to_string());
          }
      }
      None
  }

}
