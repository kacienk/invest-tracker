use jsonwebtoken::{
    decode, encode, errors::ErrorKind::ExpiredSignature, Algorithm, DecodingKey, EncodingKey,
    Header, Validation,
};

use super::models::Claims;

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
