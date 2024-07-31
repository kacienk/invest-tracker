use actix_web::web;

use super::views::get_investments;

pub fn investments_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_investments);
}
