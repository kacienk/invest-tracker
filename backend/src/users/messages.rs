use actix::Handler;
use actix::Message;
use diesel::prelude::*;
use diesel::QueryResult;
use uuid::Uuid;

use crate::common::utils::parse_uuid;
use crate::db::DBActor;
use crate::schema::investment_users::dsl::*;
use crate::users::models::{InvestmentUser, NewInvestmentUser};

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

impl Handler<GetAllInvestmentUsers> for DBActor {
    type Result = QueryResult<Vec<InvestmentUser>>;

    fn handle(&mut self, _msg: GetAllInvestmentUsers, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        investment_users
            .select(InvestmentUser::as_select())
            .load(&mut conn)
    }
}

impl Handler<GetInvestmentUser> for DBActor {
    type Result = QueryResult<InvestmentUser>;

    fn handle(&mut self, msg: GetInvestmentUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let user_id: Uuid = parse_uuid(&msg.user_id)?;
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

impl Handler<DeleteInvestmentUser> for DBActor {
    type Result = QueryResult<()>;

    fn handle(&mut self, msg: DeleteInvestmentUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let user_id: Uuid = parse_uuid(&msg.user_id)?;
        diesel::delete(investment_users.find(user_id)).execute(&mut conn)?;

        Ok(())
    }
}

impl Handler<UpdateInvestmentUser> for DBActor {
    type Result = QueryResult<InvestmentUser>;

    fn handle(&mut self, msg: UpdateInvestmentUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get DB connection");

        let user_id: Uuid = parse_uuid(&msg.user_id)?;
        diesel::update(investment_users.find(user_id))
            .set(username.eq(&msg.username))
            .returning(InvestmentUser::as_select())
            .get_result(&mut conn)
    }
}
