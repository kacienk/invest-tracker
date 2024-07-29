use std::sync::Arc;

use actix::SyncArbiter;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dashmap::DashSet;
use dotenv::dotenv;

use crate::auth::api::routes::{auth_open_routes, auth_protected_routes};
use crate::auth::middleware::auth_validator;
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
        let auth = HttpAuthentication::bearer(auth_validator);
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/api/v1/")
                    .service(web::scope("/auth").configure(auth_open_routes))
                    .service(
                        web::scope("/auth")
                            .wrap(auth)
                            .configure(auth_protected_routes),
                    )
                    .service(investments::get_investments),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
