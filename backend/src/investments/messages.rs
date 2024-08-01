use super::models::{Investment, NewInvestment};
use actix::Message;
use diesel::QueryResult;

use actix::Handler;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::DBActor;
use crate::schema::investments::dsl::*;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Investment>>")]
pub struct GetAllInvestments;

#[derive(Message)]
#[rtype(result = "QueryResult<Investment>")]
pub struct GetInvestment {
    pub investment_id: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Investment>")]
pub struct CreateInvestment<'a> {
    pub investment: NewInvestment<'a>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Investment>")]
pub struct UpdateInvestment<'a> {
    pub investment_id: String,
    pub investment: NewInvestment<'a>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<()>")]
pub struct DeleteInvestment {
    pub investment_id: String,
}

impl Handler<GetAllInvestments> for DBActor {
    type Result = QueryResult<Vec<Investment>>;

    fn handle(&mut self, _msg: GetAllInvestments, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        investments
            .select(Investment::as_select())
            .load::<Investment>(&mut conn)
    }
}

impl Handler<GetInvestment> for DBActor {
    type Result = QueryResult<Investment>;

    fn handle(&mut self, msg: GetInvestment, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        investments
            .select(Investment::as_select())
            .find(msg.id)
            .first(&mut conn)
    }
}
