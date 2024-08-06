use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind::ExpiredSignature},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

use super::super::models::Claims;

static TOKEN_VALIDITY: i64 = 15;

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: &str) -> JwtService {
        JwtService {
            secret: secret.to_string(),
        }
    }

    pub fn generate_token(&self, user_id: &str) -> String {
        let now = chrono::Utc::now();
        let issued_at = now.timestamp();
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(TOKEN_VALIDITY))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration as usize,
            iat: issued_at as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .unwrap()
    }

    pub fn validate_token(&self, token: &str) -> Result<bool, Error> {
        let validation = Validation::new(Algorithm::HS256);

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        ) {
            Ok(token_data) => {
                if token_data.claims.exp < chrono::Utc::now().timestamp() as usize {
                    return Err(Error::from(ExpiredSignature));
                }
                Ok(true)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_user_id_from_token(&self, token: &str) -> Result<String, Error> {
        let validation = Validation::new(Algorithm::HS256);

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation,
        ) {
            Ok(token_data) => Ok(token_data.claims.sub),
            Err(e) => Err(e),
        }
    }
}
