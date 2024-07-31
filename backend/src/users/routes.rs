use actix_web::web;

use super::views::{get_user, get_users};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users).service(get_user);
}
