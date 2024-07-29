use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind::ExpiredSignature, Algorithm, DecodingKey, EncodingKey,
    Header, Validation,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

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

pub fn verify_password(
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

pub fn generate_token(secret: &str, username: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn validate_token(secret: &str, token: &str) -> Result<bool, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            if token_data.claims.exp < chrono::Utc::now().timestamp() as usize {
                return Err(jsonwebtoken::errors::Error::from(ExpiredSignature));
            }
            Ok(true)
        }
        Err(e) => Err(e),
    }
}
