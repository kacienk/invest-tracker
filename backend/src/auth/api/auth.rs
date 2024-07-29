use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};

use super::errors::{AuthError, UserError};
use super::messages::{
    CreateInvestmentUser, GetAllInvestmentUsers, GetInvestmentUser, GetInvestmentUserByEmail,
};
use crate::auth::auth_utils;
use crate::auth::models::user::{CreateUserBody, InvestmentUser, NewInvestmentUser};
use crate::db::{AppState, DBActor};

#[derive(Deserialize)]
struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    pub token: String,
}

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginBody>,
) -> Result<Json<LoginResponse>, AuthError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();
    let secret: &str = state.as_ref().secret.as_ref();

    let message = GetInvestmentUserByEmail {
        email: body.email.clone(),
    };
    let user = match db.send(message).await {
        Ok(Ok(user)) => user,
        Ok(Err(_)) => return Err(AuthError::UserNotFound),
        Err(_) => return Err(AuthError::BadAuthRequest),
    };

    if auth_utils::verify_password(&body.password, &user.salt, &user.password) {
        let token = auth_utils::generate_token(secret, &user.id.to_string());
        Ok(Json(LoginResponse { token }))
    } else {
        Err(AuthError::AuthError)
    }
}

#[get("/logout")]
pub async fn logout(req: HttpRequest, state: Data<AppState>) -> Result<HttpResponse, AuthError> {
    let auth = match req.headers().get("Authorization") {
        Some(a) => a,
        None => return Err(AuthError::AuthError),
    };
    let auth_str = auth.to_str().unwrap();
    if auth_str.starts_with("Bearer ") {
        let token = &auth_str[7..];
        state.invalid_tokens.insert(token.to_string());
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(r#"{"message": "Logout successful"}"#))
    } else {
        Err(AuthError::BadAuthRequest)
    }
}

#[post("/users")]
pub async fn create_user(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Result<Json<InvestmentUser>, UserError> {
    let new_salt = auth_utils::generate_salt();
    let salt_str = general_purpose::STANDARD.encode(&new_salt);
    let hashed_password = auth_utils::hash_password(&body.password, &new_salt);
    let user = NewInvestmentUser {
        username: body.username.clone(),
        email: body.email.clone(),
        password: hashed_password,
        salt: salt_str,
        created_at: chrono::Utc::now(),
    };

    let db: Addr<DBActor> = state.as_ref().db.clone();
    match db.send(CreateInvestmentUser { user }).await {
        Ok(Ok(user)) => Ok(Json(user)),
        Ok(Err(_)) => Err(UserError::UserCreateError),
        Err(_) => Err(UserError::UserCreateError),
    }
}

#[get("/users/{user_id}")]
pub async fn get_user(
    state: Data<AppState>,
    user_id: Path<String>,
) -> Result<Json<InvestmentUser>, UserError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();
    let message = GetInvestmentUser {
        user_id: user_id.to_string(),
    };
    match db.send(message).await {
        Ok(Ok(user)) => Ok(Json(user)),
        Ok(Err(_)) => Err(UserError::UserNotFound),
        Err(_) => Err(UserError::BadUserRequest),
    }
}

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> Result<Json<Vec<InvestmentUser>>, UserError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();
    match db.send(GetAllInvestmentUsers).await {
        Ok(Ok(users)) => Ok(Json(users)),
        Ok(Err(_)) => Err(UserError::BadUserRequest),
        Err(_) => Err(UserError::BadUserRequest),
    }
}
