use crate::auth::models::user::{InvestmentUser, NewInvestmentUser};
use actix::Message;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentUser>")]
pub struct CreateInvestmentUser {
    pub user: NewInvestmentUser,
}
