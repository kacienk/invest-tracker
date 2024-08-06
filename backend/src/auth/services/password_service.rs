use base64::{engine::general_purpose, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct HashedPassword {
    pub hash: String,
    pub salt: String,
}

pub struct PasswordService {}

impl PasswordService {
    pub fn hash_password(password: &str) -> HashedPassword {
        let byte_salt: [u8; 16] = Self::generate_salt();
        let salt: String = general_purpose::STANDARD.encode(&byte_salt);
        let hash = Self::do_hash_password(password, &byte_salt);
        HashedPassword { hash, salt }
    }

    fn generate_salt() -> [u8; 16] {
        let mut rng = rand::thread_rng();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt);
        salt
    }

    fn do_hash_password(password: &str, salt: &[u8; 16]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt);
        let hash = hasher.finalize();

        general_purpose::STANDARD.encode(hash)
    }

    pub fn verify_password(
        provided_password: &str,
        stored_salt_base64: &str,
        stored_hash_base64: &str,
    ) -> bool {
        let stored_salt: [u8; 16] = general_purpose::STANDARD
            .decode(stored_salt_base64)
            .expect("Failed to decode salt")
            .try_into()
            .expect("Malformed salt");
        let hash_to_verify = Self::do_hash_password(provided_password, &stored_salt);

        hash_to_verify == stored_hash_base64
    }
}
