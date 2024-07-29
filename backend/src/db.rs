use actix::{Actor, Addr, SyncContext};
use dashmap::DashSet;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use std::sync::Arc;

pub struct DBActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Clone)]
pub struct AppState {
    pub db: Addr<DBActor>,
    pub secret: Arc<String>,
    pub invalid_tokens: Arc<DashSet<String>>,
}

impl Actor for DBActor {
    type Context = SyncContext<Self>;
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
