use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use base64::{engine::general_purpose, Engine};

use super::errors::AuthError;
use super::models::{LoginBody, TokenResponse};
use crate::{
    auth::common,
    users::{
        errors::UserError,
        messages::{CreateInvestmentUser, GetInvestmentUserByEmail},
        models::{CreateUserBody, InvestmentUserResponse, NewInvestmentUser},
    },
};

use crate::auth::utils;
use crate::db::{AppState, DBActor};

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginBody>,
) -> Result<Json<TokenResponse>, AuthError> {
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

    if utils::verify_password(&body.password, &user.salt, &user.password) {
        let token = utils::generate_token(secret, &user.id.to_string());
        Ok(Json(TokenResponse { token }))
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
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(AuthError::BadAuthRequest)
    }
}

#[post("/register")]
pub async fn register(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Result<Json<InvestmentUserResponse>, UserError> {
    let user = common::new_user(&body.username, &body.email, &body.password, false);
    let db: Addr<DBActor> = state.as_ref().db.clone();
    match db.send(CreateInvestmentUser { user }).await {
        Ok(Ok(user)) => Ok(Json(InvestmentUserResponse::from(user))),
        Ok(Err(_)) => Err(UserError::UserCreateError),
        Err(_) => Err(UserError::UserCreateError),
    }
}
