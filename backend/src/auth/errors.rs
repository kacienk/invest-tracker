use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use strum::Display;

#[derive(Debug, Display)]
pub enum AuthError {
    AuthError,
    UserNotFound,
    BadAuthRequest,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::AuthError => StatusCode::UNAUTHORIZED,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::BadAuthRequest => StatusCode::BAD_REQUEST,
        }
    }
}
