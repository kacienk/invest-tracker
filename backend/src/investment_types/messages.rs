use super::models::InvestmentType;
use actix::Handler;
use actix::Message;
use diesel::prelude::*;
use diesel::QueryResult;

use crate::db::DBActor;
use crate::schema::investment_types::dsl::*;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<InvestmentType>>")]
pub struct GetAllInvestmentTypes;

impl Handler<GetAllInvestmentTypes> for DBActor {
    type Result = QueryResult<Vec<InvestmentType>>;

    fn handle(&mut self, _msg: GetAllInvestmentTypes, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        investment_types
            .select(InvestmentType::as_select())
            .load::<InvestmentType>(&mut conn)
    }
}
