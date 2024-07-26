use crate::schema::investments;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "investments"]
pub struct NewInvestment<'a> {
    #[column_name = "investment_name"]
    pub name: &'a str,
    pub code: Option<&'a str>,
    pub initial_value: BigDecimal,
    pub current_value: BigDecimal,
    pub investment_datetime: NaiveDateTime,
    pub group_id: i32,
    pub creator_id: i32,
    pub investment_type_id: Option<i32>,
}

#[derive(Queryable, AsChangeset, Debug)]
pub struct Investment {
    pub id: i32,
    #[column_name = "investment_name"]
    pub name: String,
    pub code: Option<String>,
    pub initial_value: BigDecimal,
    pub current_value: BigDecimal,
    pub investment_datetime: NaiveDateTime,
    pub group_id: i32,
    pub creator_id: i32,
    pub investment_type_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
