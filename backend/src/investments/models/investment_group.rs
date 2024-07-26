use crate::schema::investment_groups;

use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "investment_groups"]
pub struct NewInvestmentGroup<'a> {
    #[column_name = "group_name"]
    pub name: &'a str,
    pub owner_id: i32,
}

#[derive(Queryable, AsChangeset, Debug)]
pub struct InvestmentGroup {
    pub id: i32,
    #[column_name = "group_name"]
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted: bool,
    pub owner_id: i32,
}
