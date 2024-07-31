use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::users::models::NewInvestmentUser;

use super::{models::Claims, utils};

pub fn get_username_from_token(
    secret: &str,
    token: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(e) => Err(e),
    }
}

pub fn new_user(
    username: &String,
    email: &String,
    passwd: &String,
    superuser: bool,
) -> NewInvestmentUser {
    let new_salt = utils::generate_salt();
    let salt_str = general_purpose::STANDARD.encode(&new_salt);
    let hashed_password = utils::hash_password(passwd, &new_salt);

    NewInvestmentUser {
        username: username.clone(),
        email: email.clone(),
        password: hashed_password,
        salt: salt_str,
        superuser: superuser.clone(),
    }
}
