use actix::{Actor, Addr, SyncContext};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

pub struct DBActor(pub Pool<ConnectionManager<PgConnection>>);

pub struct AppState {
    pub db: Addr<DBActor>,
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
