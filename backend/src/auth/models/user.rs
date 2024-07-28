use crate::schema::investment_users;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "investment_users"]
pub struct NewInvestmentUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, AsChangeset, Serialize, Debug)]
pub struct InvestmentUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: DateTime<Utc>,
}
