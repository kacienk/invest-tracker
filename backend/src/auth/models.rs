use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}
