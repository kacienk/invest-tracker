use actix::Addr;
use actix_web::{error::ResponseError, get, http::StatusCode, web::Data, web::Json, HttpResponse};
use strum::Display;

use super::models::Investment;
use crate::{
    db::{AppState, DBActor},
    investments::messages::GetInvestments,
};

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

#[get("/investments")]
pub async fn get_investments(
    state: Data<AppState>,
) -> Result<Json<Vec<Investment>>, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    match db.send(GetInvestments).await {
        Ok(Ok(investments)) => Ok(Json(investments)),
        _ => Err(InvestmentsError::InvestmentNotFound),
    }
}
