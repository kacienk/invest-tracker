use crate::auth::models::user::{InvestmentUser, NewInvestmentUser};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentUser>")]
pub struct CreateInvestmentUser {
    pub user: NewInvestmentUser,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<InvestmentUser>>")]
pub struct GetAllInvestmentUsers;

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentUser>")]
pub struct GetInvestmentUser {
    pub user_id: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentUser>")]
pub struct GetInvestmentUserByEmail {
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentUser>")]
pub struct UpdateInvestmentUser {
    pub user_id: String,
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<()>")]
pub struct DeleteInvestmentUser {
    pub user_id: String,
}
