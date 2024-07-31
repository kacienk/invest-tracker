use actix::Addr;
use actix_web::get;
use actix_web::web::{Data, Json, Path};

use super::errors::UserError;
use super::messages::{GetAllInvestmentUsers, GetInvestmentUser};
use super::models::InvestmentUserResponse;
use crate::db::{AppState, DBActor};

#[get("/users/{user_id}")]
pub async fn get_user(
    state: Data<AppState>,
    user_id: Path<String>,
) -> Result<Json<InvestmentUserResponse>, UserError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();
    let message = GetInvestmentUser {
        user_id: user_id.to_string(),
    };
    match db.send(message).await {
        Ok(Ok(user)) => Ok(Json(InvestmentUserResponse::from(user))),
        Ok(Err(_)) => Err(UserError::UserNotFound),
        Err(_) => Err(UserError::BadUserRequest),
    }
}

#[get("/users")]
pub async fn get_users(
    state: Data<AppState>,
) -> Result<Json<Vec<InvestmentUserResponse>>, UserError> {
    let db: Addr<DBActor> = state.as_ref().db.clone();
    match db.send(GetAllInvestmentUsers).await {
        Ok(Ok(users)) => {
            let response_users: Vec<InvestmentUserResponse> = users
                .into_iter()
                .map(InvestmentUserResponse::from)
                .collect();
            Ok(Json(response_users))
        }
        Ok(Err(_)) => Err(UserError::BadUserRequest),
        Err(_) => Err(UserError::BadUserRequest),
    }
}
