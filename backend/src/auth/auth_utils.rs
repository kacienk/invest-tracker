use std::env;

use actix::Addr;
use actix_web::dev::ServiceRequest;
use actix_web::web::{Data, Json};
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use base64::{engine::general_purpose, Engine};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind::ExpiredSignature, Algorithm, DecodingKey, EncodingKey,
    Header, Validation,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::db::{AppState, DBActor};

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

fn validate_token(secret: &str, token: &str) -> Result<bool, jsonwebtoken::errors::Error> {
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

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    dotenv().ok();
    let data: &Data<AppState> = req.app_data::<Data<AppState>>().unwrap();
    let secret: &String = &data.secret;
    let token = credentials.token();

    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    if data.invalid_tokens.contains(token) {
        return Err(AuthenticationError::from(config).into());
    }

    match validate_token(secret, token) {
        Ok(res) => {
            if res {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
