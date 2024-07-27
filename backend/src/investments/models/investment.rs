use crate::schema::investments;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{pg::Pg, prelude::*};
use serde::Serialize;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "investments"]
pub struct NewInvestment<'a> {
    #[column_name = "investment_name"]
    pub name: &'a str,
    pub code: Option<&'a str>,
    pub initial_value: BigDecimal,
    pub current_value: BigDecimal,
    pub investment_datetime: NaiveDateTime,
    pub group_id: Uuid,
    pub creator_id: Uuid,
    pub investment_type_id: Option<Uuid>,
}

#[derive(Selectable, Queryable, Serialize, Debug)]
#[diesel(check_for_backend(Pg))]
pub struct Investment {
    pub id: Uuid,
    #[column_name = "investment_name"]
    pub name: String,
    pub code: Option<String>,
    pub initial_value: BigDecimal,
    pub current_value: BigDecimal,
    pub investment_datetime: NaiveDateTime,
    pub group_id: Uuid,
    pub creator_id: Uuid,
    pub investment_type_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
