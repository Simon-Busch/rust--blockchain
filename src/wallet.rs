use secp256k1::{PublicKey, Secp256k1, SecretKey};
use rand::rngs::OsRng;
use rand::RngCore;

#[derive(Debug, Clone, Copy)]
pub struct Wallet {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl Wallet {
    pub fn new() -> Self {
        let (secret_key, public_key) = Self::generate_keypair();
        Wallet {
            public_key,
            secret_key,
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
