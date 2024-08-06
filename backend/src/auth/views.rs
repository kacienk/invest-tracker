use actix::Addr;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::common;
use super::errors::AuthError;
use super::models::{LoginBody, TokenResponse};
use super::services::{jwt_service::JwtService, password_service::PasswordService};
use crate::db::{AppState, DBActor};
use crate::users::{
    errors::UserError,
    messages::{CreateInvestmentUser, GetInvestmentUserByEmail},
    models::{CreateUserBody, InvestmentUserResponse},
};

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginBody>,
) -> Result<HttpResponse, AuthError> {
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

    if PasswordService::verify_password(&body.password, &user.salt, &user.password) {
        let token_service = JwtService::new(secret);
        let token: String = token_service.generate_token(&user.id.to_string());
        Ok(HttpResponse::Ok().json(TokenResponse { token }))
    } else {
        Err(AuthError::AuthError)
    }
}

#[get("/logout")]
pub async fn logout(state: Data<AppState>, credentials: BearerAuth) -> HttpResponse {
    let token = credentials.token();
    state.invalid_tokens.insert(token.to_string());
    HttpResponse::Ok().finish()
}

#[post("/register")]
pub async fn register(
    state: Data<AppState>,
    body: Json<CreateUserBody>,
) -> Result<HttpResponse, UserError> {
    let user = common::new_user(&body.username, &body.email, &body.password, false);
    let db: Addr<DBActor> = state.as_ref().db.clone();
    match db.send(CreateInvestmentUser { user }).await {
        Ok(Ok(user)) => Ok(HttpResponse::Created().json(InvestmentUserResponse::from(user))),
        Ok(Err(_)) => Err(UserError::UserCreateError),
        Err(_) => Err(UserError::UserCreateError),
    }
}
