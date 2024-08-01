use super::models::{Investment, InvestmentUpdate, NewInvestment};
use actix::Message;
use diesel::QueryResult;

use actix::Handler;
use diesel::prelude::*;
use uuid::Uuid;

use crate::common::utils::parse_uuid;
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
pub struct CreateInvestment {
    pub new_investment: NewInvestment,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Investment>")]
pub struct UpdateInvestment {
    pub investment_id: String,
    pub investment: InvestmentUpdate,
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

        let investment_id: Uuid = parse_uuid(&msg.investment_id)?;
        investments
            .select(Investment::as_select())
            .find(investment_id)
            .first(&mut conn)
    }
}

impl Handler<CreateInvestment> for DBActor {
    type Result = QueryResult<Investment>;

    fn handle(&mut self, msg: CreateInvestment, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        diesel::insert_into(investments)
            .values(&msg.new_investment)
            .returning(Investment::as_select())
            .get_result(&mut conn)
    }
}

impl Handler<UpdateInvestment> for DBActor {
    type Result = QueryResult<Investment>;

    fn handle(&mut self, msg: UpdateInvestment, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let investment_id: Uuid = parse_uuid(&msg.investment_id)?;
        diesel::update(investments.find(investment_id))
            .set(&msg.investment)
            .returning(Investment::as_select())
            .get_result(&mut conn)
    }
}

impl Handler<DeleteInvestment> for DBActor {
    type Result = QueryResult<()>;

    fn handle(&mut self, msg: DeleteInvestment, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let investment_id: Uuid = parse_uuid(&msg.investment_id)?;
        diesel::delete(investments.find(investment_id)).execute(&mut conn)?;

        Ok(())
    }
}
