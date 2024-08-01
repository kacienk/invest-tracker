use actix_web::web;

use super::views::get_all_investment_types;

pub fn investment_types_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_investment_types);
}
