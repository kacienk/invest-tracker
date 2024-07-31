use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use strum::Display;

#[derive(Debug, Display)]
pub enum InvestmentsError {
    InvestmentNotFound,
    InvestmentUpdateError,
    InvestmentCreateError,
    InvestmentDeleteError,
    BadInvestmentRequest,
}

impl ResponseError for InvestmentsError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            InvestmentsError::InvestmentNotFound => StatusCode::NOT_FOUND,
            InvestmentsError::InvestmentUpdateError => StatusCode::INTERNAL_SERVER_ERROR,
            InvestmentsError::InvestmentCreateError => StatusCode::INTERNAL_SERVER_ERROR,
            InvestmentsError::InvestmentDeleteError => StatusCode::INTERNAL_SERVER_ERROR,
            InvestmentsError::BadInvestmentRequest => StatusCode::BAD_REQUEST,
        }
    }
}
