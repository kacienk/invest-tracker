use actix_web::web;

use super::views::{
    create_investment, delete_investment, get_all_investments, get_investment, update_investment,
};

pub fn investments_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_investments)
        .service(get_investment)
        .service(create_investment)
        .service(update_investment)
        .service(delete_investment);
}
