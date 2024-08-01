use actix::Addr;
use actix_web::get;
use actix_web::web::{Data, Json, Path};

use super::errors::InvestmentsError;
use super::messages::{GetAllInvestments, GetInvestment};
use super::models::Investment;
use crate::db::{AppState, DBActor};

#[get("/investments")]
pub async fn get_investments(
    state: Data<AppState>,
) -> Result<Json<Vec<Investment>>, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    match db.send(GetAllInvestments).await {
        Ok(Ok(investments)) => Ok(Json(investments)),
        _ => Err(InvestmentsError::InvestmentNotFound),
    }
}

#[get("/investments/{id}")]
pub async fn get_investment(
    state: Data<AppState>,
    id: Path<String>,
) -> Result<Json<Investment>, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = GetInvestment {
        investment_id: id.into_inner(),
    };
    match db.send(message).await {
        Ok(Ok(investment)) => Ok(Json(investment)),
        _ => Err(InvestmentsError::InvestmentNotFound),
    }
}
