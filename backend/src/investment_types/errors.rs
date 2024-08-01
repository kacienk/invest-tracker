use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use strum::Display;

#[derive(Debug, Display)]
pub enum InvestmentTypesError {
    UnknownInvestmentTypeError,
    BadInvestmentTypesRequest,
}

impl ResponseError for InvestmentTypesError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            InvestmentTypesError::UnknownInvestmentTypeError => StatusCode::INTERNAL_SERVER_ERROR,
            InvestmentTypesError::BadInvestmentTypesRequest => StatusCode::BAD_REQUEST,
        }
    }
}
