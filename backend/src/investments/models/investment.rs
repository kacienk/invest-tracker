#[macro_use]
extern crate diesel;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "investments")]
struct Investment {
    id: u64,
    name: String,
    #[serde(rename = "type")]
    investment_type: String,
    #[serde(rename = "amount")]
    investment_amount: f64,
    #[serde(rename = "datetime")]
    investment_datetime: String,
    #[serde(rename = "user_id")]
    user_id: u64,
}
