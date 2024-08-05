use crate::{schema::investment_groups, users::models::InvestmentUser};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "investment_groups"]
pub struct NewInvestmentGroup {
    #[column_name = "group_name"]
    pub name: String,
    pub owner_id: Uuid,
}

#[derive(Identifiable, Queryable, Associations, Selectable, Debug, Serialize)]
#[diesel(belongs_to(InvestmentUser, foreign_key = owner_id))]
pub struct InvestmentGroup {
    pub id: Uuid,
    #[column_name = "group_name"]
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
    pub owner_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInvestmentGroupRequestBody {
    pub name: Option<String>,
    pub deleted: Option<bool>,
}

#[derive(AsChangeset)]
#[table_name = "investment_groups"]
pub struct InvestmentGroupUpdate {
    #[column_name = "group_name"]
    pub name: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub deleted: Option<bool>,
}

impl From<UpdateInvestmentGroupRequestBody> for InvestmentGroupUpdate {
    fn from(body: UpdateInvestmentGroupRequestBody) -> Self {
        InvestmentGroupUpdate {
            name: body.name,
            updated_at: Utc::now(),
            deleted: body.deleted,
        }
    }
}
