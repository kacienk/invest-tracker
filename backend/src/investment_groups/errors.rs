use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use strum::Display;

#[derive(Debug, Display)]
pub enum InvestmentGroupError {
    InvestmentGroupNotFound,
    InvestmentGroupUpdateError,
    InvestmentGroupCreateError,
    InvestmentGroupDeleteError,
    BadInvestmentGroupRequest,
}

impl ResponseError for InvestmentGroupError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            InvestmentGroupError::InvestmentGroupNotFound => StatusCode::NOT_FOUND,
            InvestmentGroupError::InvestmentGroupUpdateError => StatusCode::INTERNAL_SERVER_ERROR,
            InvestmentGroupError::InvestmentGroupCreateError => StatusCode::INTERNAL_SERVER_ERROR,
            InvestmentGroupError::InvestmentGroupDeleteError => StatusCode::INTERNAL_SERVER_ERROR,
            InvestmentGroupError::BadInvestmentGroupRequest => StatusCode::BAD_REQUEST,
        }
    }
}
