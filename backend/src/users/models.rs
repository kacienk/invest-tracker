use crate::{
    auth::services::password_service::{HashedPassword, PasswordService},
    schema::investment_users,
};

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
    pub superuser: bool,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Serialize, Debug)]
pub struct InvestmentUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub superuser: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct InvestmentUserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl NewInvestmentUser {
    pub fn new(username: &str, email: &str, passwd: &str, superuser: bool) -> NewInvestmentUser {
        let HashedPassword { hash, salt } = PasswordService::hash_password(passwd);

        NewInvestmentUser {
            username: username.to_owned(),
            email: email.to_owned(),
            password: hash,
            salt,
            superuser,
        }
    }
}

impl From<InvestmentUser> for InvestmentUserResponse {
    fn from(value: InvestmentUser) -> Self {
        InvestmentUserResponse {
            id: value.id,
            username: value.username,
            email: value.email,
            created_at: value.created_at,
        }
    }
}
