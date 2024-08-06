use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind::ExpiredSignature, Algorithm, DecodingKey, EncodingKey,
    Header, Validation,
};
use rand::Rng;
use sha2::{Digest, Sha256};

struct HashedPassword {
    hash: String,
    salt: String,
}

enum PasswordServiceState {
    Hashed,
    Unhashed,
}

pub struct PasswordService {
    password: Option<String>,
    hash: Option<String>,
    salt: [u8; 16],
    state: PasswordServiceState,
}

impl PasswordService {
    pub fn from_hashed(hash_base64: &str, salt_base64: &str) -> PasswordService {
        let stored_salt = general_purpose::STANDARD
            .decode(salt_base64)
            .expect("Failed to decode salt");

        PasswordService {
            password: None,
            hash: Some(hash_base64.to_string()),
            salt: stored_salt,
            state: PasswordServiceState::Hashed,
        }
    }

    pub fn from_unhashed(password: &str) -> PasswordService {
        PasswordService {
            password: Some(password.to_string()),
            hash: None,
            salt: generate_salt(),
            state: PasswordServiceState::Unhashed,
        }
    }

    fn generate_salt() -> [u8; 16] {
        let mut rng = rand::thread_rng();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt);
        salt
    }

    pub fn get_hashed_password(&mut self) -> HashedPassword {
        match self.state {
            PasswordServiceState::Hashed => HashedPassword {
                hash: self.hash.clone().unwrap(),
                salt: general_purpose::STANDARD.encode(&self.salt),
            },
            PasswordServiceState::Unhashed => {
                let hash = hash_password(self.password.clone().unwrap(), &self.salt);
                self.hash = Some(hash.clone());
                self.state = PasswordServiceState::Hashed;
                HashedPassword {
                    hash,
                    salt: general_purpose::STANDARD.encode(&self.salt),
                }
            }
        }
    }

    fn hash_password(password: &str, salt: &[u8; 16]) -> String {
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
        let hash_to_verify = hash_password(provided_password, &stored_salt);

        hash_to_verify == stored_hash_base64
    }
}
