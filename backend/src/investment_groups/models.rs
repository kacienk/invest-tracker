use crate::schema::investment_groups;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "investment_groups"]
pub struct NewInvestmentGroup<'a> {
    #[column_name = "group_name"]
    pub name: &'a str,
    pub owner_id: Uuid,
}

#[derive(Queryable, AsChangeset, Debug)]
pub struct InvestmentGroup {
    pub id: Uuid,
    #[column_name = "group_name"]
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub owner_id: Uuid,
}
