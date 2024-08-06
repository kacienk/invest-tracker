use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::users::models::NewInvestmentUser;

use super::{
    models::Claims,
    services::password_service::{HashedPassword, PasswordService},
    utils,
};

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
    let HashedPassword { hash, salt } = PasswordService::hash_password(passwd);

    NewInvestmentUser {
        username: username.clone(),
        email: email.clone(),
        password: hash,
        salt,
        superuser: superuser.clone(),
    }
}
