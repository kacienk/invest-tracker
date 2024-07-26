use crate::schema::investment_users;

use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "investment_users"]
pub struct NewInvestmentUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub salt: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, AsChangeset, Debug)]
pub struct InvestmentUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: NaiveDateTime,
}
