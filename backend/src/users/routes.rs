use actix_web::web;

use super::views::{get_investments_for_user, get_user, get_users, get_users_investment_groups};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user)
        .service(get_investments_for_user)
        .service(get_users_investment_groups);
}
