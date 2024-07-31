use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use strum::Display;

#[derive(Debug, Display)]
pub enum UserError {
    UserNotFound,
    UserUpdateError,
    UserCreateError,
    UserDeleteError,
    BadUserRequest,
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
