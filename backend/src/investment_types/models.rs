use crate::schema::investment_types;

use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "investment_types"]
pub struct NewInvestmentType<'a> {
    #[column_name = "type_name"]
    pub name: &'a str,
}

#[derive(Queryable, Debug)]
pub struct InvestmentType {
    pub id: Uuid,
    #[column_name = "type_name"]
    pub name: String,
}
