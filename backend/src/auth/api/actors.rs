use actix::Handler;
use diesel::prelude::*;

use crate::auth::api::messages::CreateInvestmentUser;
use crate::auth::models::user::InvestmentUser;
use crate::db::DBActor;
use crate::schema::investment_users::dsl::*;

impl Handler<CreateInvestmentUser> for DBActor {
    type Result = QueryResult<InvestmentUser>;

    fn handle(&mut self, msg: CreateInvestmentUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let new_user = msg.user;
        diesel::insert_into(investment_users)
            .values(&new_user)
            .returning(InvestmentUser::as_select())
            .get_result(&mut conn)
    }
}
