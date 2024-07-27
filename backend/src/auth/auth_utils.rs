use base64::{engine::general_purpose, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};

pub fn generate_salt() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt);
    salt
}

pub fn hash_password(password: &str, salt: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let hash = hasher.finalize();

    general_purpose::STANDARD.encode(hash)
}

fn verify_password(
    provided_password: &str,
    stored_salt_base64: &str,
    stored_hash_base64: &str,
) -> bool {
    let stored_salt = general_purpose::STANDARD
        .decode(stored_salt_base64)
        .expect("Failed to decode salt");
    let hash_to_verify = hash_password(provided_password, &stored_salt);

    hash_to_verify == stored_hash_base64
}
