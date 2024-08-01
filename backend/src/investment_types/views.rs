use actix::Addr;
use actix_web::web::Data;
use actix_web::{get, HttpResponse};

use super::errors::InvestmentTypesError;
use super::messages::GetAllInvestmentTypes;

use crate::db::{AppState, DBActor};

#[get("/investments")]
pub async fn get_all_investment_types(
    state: Data<AppState>,
) -> Result<HttpResponse, InvestmentTypesError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    match db.send(GetAllInvestmentTypes).await {
        Ok(Ok(investment_types)) => Ok(HttpResponse::Ok().json(investment_types)),
        _ => Err(InvestmentTypesError::UnknownInvestmentTypeError),
    }
}
