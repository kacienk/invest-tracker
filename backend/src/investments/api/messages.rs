use crate::investments::models::investment::Investment;
use actix::Message;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Investment>>")]
pub struct GetInvestments;
