use actix::Handler;
use diesel::prelude::*;
use uuid::Uuid;

use crate::auth::api::messages::{
    CreateInvestmentUser, GetInvestmentUser, GetInvestmentUserByEmail,
};
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

impl Handler<GetInvestmentUser> for DBActor {
    type Result = QueryResult<InvestmentUser>;

    fn handle(&mut self, msg: GetInvestmentUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let user_id: Uuid = match Uuid::parse_str(&msg.user_id) {
            Ok(id_) => id_,
            Err(_) => return Err(diesel::result::Error::NotFound),
        };

        investment_users
            .select(InvestmentUser::as_select())
            .find(user_id)
            .first(&mut conn)
    }
}

impl Handler<GetInvestmentUserByEmail> for DBActor {
    type Result = QueryResult<InvestmentUser>;

    fn handle(&mut self, msg: GetInvestmentUserByEmail, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        investment_users
            .select(InvestmentUser::as_select())
            .filter(email.eq(&msg.email))
            .first(&mut conn)
    }
}
