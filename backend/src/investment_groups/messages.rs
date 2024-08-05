use super::models::{InvestmentGroup, InvestmentGroupUpdate, NewInvestmentGroup};
use actix::Message;
use diesel::QueryResult;

use actix::Handler;
use diesel::prelude::*;
use uuid::Uuid;

use crate::common::utils::parse_uuid;
use crate::db::DBActor;
use crate::schema::investment_groups::dsl::*;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<InvestmentGroup>>")]
pub struct GetAllInvestmentGroups;

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentGroup>")]
pub struct GetInvestmentGroup {
    pub investment_group_id: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentGroup>")]
pub struct CreateInvestmentGroup {
    pub new_investment_group: NewInvestmentGroup,
}

#[derive(Message)]
#[rtype(result = "QueryResult<InvestmentGroup>")]
pub struct UpdateInvestmentGroup {
    pub investment_group_id: String,
    pub investment_group: InvestmentGroupUpdate,
}

#[derive(Message)]
#[rtype(result = "QueryResult<()>")]
pub struct DeleteInvestmentGroup {
    pub investment_group_id: String,
}

impl Handler<GetAllInvestmentGroups> for DBActor {
    type Result = QueryResult<Vec<InvestmentGroup>>;

    fn handle(&mut self, _msg: GetAllInvestmentGroups, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        investment_groups
            .select(InvestmentGroup::as_select())
            .load::<InvestmentGroup>(&mut conn)
    }
}

impl Handler<GetInvestmentGroup> for DBActor {
    type Result = QueryResult<InvestmentGroup>;

    fn handle(&mut self, msg: GetInvestmentGroup, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let investment_group_id: Uuid = parse_uuid(&msg.investment_group_id)?;
        investment_groups
            .select(InvestmentGroup::as_select())
            .find(investment_group_id)
            .first(&mut conn)
    }
}

impl Handler<CreateInvestmentGroup> for DBActor {
    type Result = QueryResult<InvestmentGroup>;

    fn handle(&mut self, msg: CreateInvestmentGroup, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        diesel::insert_into(investment_groups)
            .values(&msg.new_investment_group)
            .returning(InvestmentGroup::as_select())
            .get_result(&mut conn)
    }
}

impl Handler<UpdateInvestmentGroup> for DBActor {
    type Result = QueryResult<InvestmentGroup>;

    fn handle(&mut self, msg: UpdateInvestmentGroup, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let investment_group_id: Uuid = parse_uuid(&msg.investment_group_id)?;
        diesel::update(investment_groups.find(investment_group_id))
            .set(&msg.investment_group)
            .returning(InvestmentGroup::as_select())
            .get_result(&mut conn)
    }
}

impl Handler<DeleteInvestmentGroup> for DBActor {
    type Result = QueryResult<()>;

    fn handle(&mut self, msg: DeleteInvestmentGroup, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let investment_group_id: Uuid = parse_uuid(&msg.investment_group_id)?;
        diesel::delete(investment_groups.find(investment_group_id)).execute(&mut conn)?;

        Ok(())
    }
}
