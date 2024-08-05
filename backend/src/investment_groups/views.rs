use actix::Addr;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, patch, post, HttpResponse};

use super::errors::InvestmentGroupError;
use super::messages::{
    CreateInvestmentGroup, DeleteInvestmentGroup, GetAllInvestmentGroups, GetInvestmentGroup,
    UpdateInvestmentGroup,
};

use super::models::{InvestmentGroupUpdate, NewInvestmentGroup, UpdateInvestmentGroupRequestBody};
use crate::db::{AppState, DBActor};

#[get("/investment_groups")]
pub async fn get_all_investment_groups(
    state: Data<AppState>,
) -> Result<HttpResponse, InvestmentGroupError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    match db.send(GetAllInvestmentGroups).await {
        Ok(Ok(investment_group)) => Ok(HttpResponse::Ok().json(investment_group)),
        _ => Err(InvestmentGroupError::InvestmentGroupNotFound),
    }
}

#[get("/investment_groups/{id}")]
pub async fn get_investment_group(
    state: Data<AppState>,
    id: Path<String>,
) -> Result<HttpResponse, InvestmentGroupError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = GetInvestmentGroup {
        investment_group_id: id.into_inner(),
    };
    match db.send(message).await {
        Ok(Ok(investment)) => Ok(HttpResponse::Ok().json(investment)),
        _ => Err(InvestmentGroupError::InvestmentGroupNotFound),
    }
}

#[post("/investment_groups")]
pub async fn create_investment_group(
    state: Data<AppState>,
    body: Json<NewInvestmentGroup>,
) -> Result<HttpResponse, InvestmentGroupError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = CreateInvestmentGroup {
        new_investment_group: body.into_inner(),
    };
    match db.send(message).await {
        Ok(Ok(investment)) => Ok(HttpResponse::Created().json(investment)),
        _ => Err(InvestmentGroupError::InvestmentGroupNotFound),
    }
}

#[patch("/investments/{id}")]
pub async fn update_investment_group(
    state: Data<AppState>,
    id: Path<String>,
    body: Json<UpdateInvestmentGroupRequestBody>,
) -> Result<HttpResponse, InvestmentGroupError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = UpdateInvestmentGroup {
        investment_group_id: id.into_inner(),
        investment_group: InvestmentGroupUpdate::from(body.into_inner()),
    };
    match db.send(message).await {
        Ok(Ok(investment)) => Ok(HttpResponse::Ok().json(investment)),
        Ok(Err(_)) => Err(InvestmentGroupError::InvestmentGroupNotFound),
        Err(_) => Err(InvestmentGroupError::InvestmentGroupUpdateError),
    }
}

#[delete("/investment_groups/{id}")]
pub async fn delete_investment_group(
    state: Data<AppState>,
    id: Path<String>,
) -> Result<HttpResponse, InvestmentGroupError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();

    let message = DeleteInvestmentGroup {
        investment_group_id: id.into_inner(),
    };
    match db.send(message).await {
        Ok(Ok(_)) => Ok(HttpResponse::NoContent().finish()),
        Ok(Err(_)) => Err(InvestmentGroupError::InvestmentGroupNotFound),
        Err(_) => Err(InvestmentGroupError::InvestmentGroupDeleteError),
    }
}
