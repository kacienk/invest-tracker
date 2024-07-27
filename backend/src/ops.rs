use actix::SyncArbiter;
use actix_web::{middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;

use crate::db::{get_pool, AppState, DBActor};
use crate::investments::api::investments;

pub async fn run_server() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&db_url);
    let db_actor = SyncArbiter::start(3, move || DBActor(pool.clone()));

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(AppState {
                db: db_actor.clone(),
            }))
            .service(investments::get_investments)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
