use actix_web::{middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};

use crate::investments::api::investments;

pub async fn run_server() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(investments::get_investments)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
