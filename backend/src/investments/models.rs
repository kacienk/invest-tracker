use crate::schema::investments;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "investments"]
pub struct NewInvestment {
    #[column_name = "investment_name"]
    pub name: String,
    pub code: Option<String>,
    pub initial_value: BigDecimal,
    pub current_value: BigDecimal,
    pub investment_datetime: DateTime<Utc>,
    pub group_id: Uuid,
    pub creator_id: Uuid,
    pub investment_type_id: Option<Uuid>,
}

#[derive(AsChangeset)]
#[table_name = "investments"]
pub struct InvestmentUpdate {
    #[column_name = "investment_name"]
    pub name: Option<String>,
    pub code: Option<String>,
    pub initial_value: Option<BigDecimal>,
    pub current_value: Option<BigDecimal>,
    pub investment_datetime: Option<DateTime<Utc>>,
    pub group_id: Option<Uuid>,
    pub investment_type_id: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
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
    pub investment_datetime: DateTime<Utc>,
    pub group_id: Uuid,
    pub creator_id: Uuid,
    pub investment_type_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInvestmentRequestBody {
    pub name: Option<String>,
    pub code: Option<String>,
    pub initial_value: Option<BigDecimal>,
    pub current_value: Option<BigDecimal>,
    pub investment_datetime: Option<DateTime<Utc>>,
    pub group_id: Option<Uuid>,
    pub investment_type_id: Option<Uuid>,
}

impl From<UpdateInvestmentRequestBody> for InvestmentUpdate {
    fn from(body: UpdateInvestmentRequestBody) -> Self {
        InvestmentUpdate {
            name: body.name,
            code: body.code,
            initial_value: body.initial_value,
            current_value: body.current_value,
            investment_datetime: body.investment_datetime,
            group_id: body.group_id,
            investment_type_id: body.investment_type_id,
            updated_at: Utc::now(),
        }
    }
}
