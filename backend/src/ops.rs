use std::sync::Arc;

use actix::SyncArbiter;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dashmap::DashSet;
use dotenv::dotenv;

use crate::db::{get_pool, AppState, DBActor};
use crate::investments::api::investments;

pub async fn run_server() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = get_pool(&db_url);
    let db_actor = SyncArbiter::start(3, move || DBActor(pool.clone()));
    let state = AppState {
        db: db_actor.clone(),
        secret: Arc::new(secret),
        invalid_tokens: Arc::new(DashSet::new()),
    };

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(state.clone()))
            .service(investments::get_investments)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
