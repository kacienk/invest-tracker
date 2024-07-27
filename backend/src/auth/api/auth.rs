use actix::Addr;
use actix_web::{
    body,
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use base64::{engine::general_purpose, Engine};
use strum::Display;

use crate::auth::models::user::{InvestmentUser, NewInvestmentUser};
use crate::auth::{auth_utils, models::user::CreateUserBody};
use crate::{
    auth::api::messages::CreateInvestmentUser,
    db::{AppState, DBActor},
};

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    UserUpdateError,
    UserCreateError,
    UserDeleteError,
    BadUserRequest,
}

#[derive(Debug, Display)]
pub enum AuthError {
    AuthError,
    BadAuthRequest,
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserUpdateError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UserCreateError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UserDeleteError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::BadUserRequest => StatusCode::BAD_REQUEST,
        }
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::AuthError => StatusCode::UNAUTHORIZED,
            AuthError::BadAuthRequest => StatusCode::BAD_REQUEST,
        }
    }
}

#[post("/auth/user")]
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
        created_at: chrono::Local::now().naive_local(),
    };

    let db: Addr<DBActor> = state.as_ref().db.clone();
    match db.send(CreateInvestmentUser { user }).await {
        Ok(Ok(user)) => Ok(Json(user)),
        Ok(Err(_)) => Err(UserError::UserCreateError),
        Err(_) => Err(UserError::UserCreateError),
    }
}
