use secp256k1::{PublicKey, Secp256k1, SecretKey};
use rand::rngs::OsRng;
use rand::RngCore;

#[derive(Debug)]
pub struct Wallet {
    pub public_key: String,
    pub secret_key: String,
}

impl Wallet {
  pub fn new() -> Self {
    let (secret_key, public_key) = Self::generate_keypair();

    // Convert the keys to hexadecimal strings for storage in the Wallet struct
    let secret_hex = bytes_to_hex(&secret_key[..]);
    let public_hex = bytes_to_hex(&public_key.serialize());

    Wallet {
        public_key: public_hex,
        secret_key: secret_hex,
    }
}

    pub fn generate_keypair() -> (SecretKey, PublicKey) {
      let secp = Secp256k1::new();
      let mut rng = OsRng;

      // Generate a random array of bytes to be used as a secret key
      let mut key_data = [0u8; 32];
      rng.fill_bytes(&mut key_data);

      // Create a secret key from the generated random data
      let secret_key = SecretKey::from_slice(&key_data).expect("Invalid key data");

      // Derive the corresponding public key
      let public_key = PublicKey::from_secret_key(&secp, &secret_key);

      (secret_key, public_key)
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
  let hex_chars: Vec<String> = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
  hex_chars.join("")
}
