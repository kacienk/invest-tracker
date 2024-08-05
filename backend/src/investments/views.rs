use actix::Addr;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, patch, post, HttpResponse};

use super::errors::InvestmentsError;
use super::messages::{
    CreateInvestment, DeleteInvestment, GetAllInvestments, GetInvestment, UpdateInvestment,
};
use super::models::{InvestmentUpdate, NewInvestment};
use crate::db::{AppState, DBActor};
use crate::investments::models::UpdateInvestmentRequestBody;

#[get("/investments")]
pub async fn get_all_investments(state: Data<AppState>) -> Result<HttpResponse, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    match db.send(GetAllInvestments).await {
        Ok(Ok(investments)) => Ok(HttpResponse::Ok().json(investments)),
        _ => Err(InvestmentsError::InvestmentNotFound),
    }
}

#[get("/investments/{id}")]
pub async fn get_investment(
    state: Data<AppState>,
    id: Path<String>,
) -> Result<HttpResponse, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = GetInvestment {
        investment_id: id.into_inner(),
    };
    match db.send(message).await {
        Ok(Ok(investment)) => Ok(HttpResponse::Ok().json(investment)),
        _ => Err(InvestmentsError::InvestmentNotFound),
    }
}

#[post("/investments")]
pub async fn create_investment(
    state: Data<AppState>,
    body: Json<NewInvestment>,
) -> Result<HttpResponse, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = CreateInvestment {
        new_investment: body.into_inner(),
    };
    match db.send(message).await {
        Ok(Ok(investment)) => Ok(HttpResponse::Created().json(investment)),
        _ => Err(InvestmentsError::InvestmentNotFound),
    }
}

#[patch("/investments/{id}")]
pub async fn update_investment(
    state: Data<AppState>,
    id: Path<String>,
    body: Json<UpdateInvestmentRequestBody>,
) -> Result<HttpResponse, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = UpdateInvestment {
        investment_id: id.into_inner(),
        investment: InvestmentUpdate::from(body.into_inner()),
    };
    match db.send(message).await {
        Ok(Ok(investment)) => Ok(HttpResponse::Ok().json(investment)),
        Ok(Err(_)) => Err(InvestmentsError::InvestmentNotFound),
        Err(_) => Err(InvestmentsError::InvestmentUpdateError),
    }
}

#[delete("/investments/{id}")]
pub async fn delete_investment(
    state: Data<AppState>,
    id: Path<String>,
) -> Result<HttpResponse, InvestmentsError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = DeleteInvestment {
        investment_id: id.into_inner(),
    };
    match db.send(message).await {
        Ok(Ok(_)) => Ok(HttpResponse::NoContent().finish()),
        Ok(Err(_)) => Err(InvestmentsError::InvestmentNotFound),
        Err(_) => Err(InvestmentsError::InvestmentDeleteError),
    }
}
