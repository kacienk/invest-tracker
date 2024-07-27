use actix::{Addr, Handler};
use diesel::prelude::*;

use crate::db::DBActor;
use crate::investments::api::messages::GetInvestments;
use crate::investments::models::investment::Investment;
use crate::schema::investments::dsl::*;

impl Handler<GetInvestments> for DBActor {
    type Result = QueryResult<Vec<Investment>>;

    fn handle(&mut self, _msg: GetInvestments, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        investments
            .select(Investment::as_select())
            .load::<Investment>(&mut conn)
    }
}
