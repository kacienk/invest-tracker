use actix_web::web;

use crate::auth::views::{login, logout, register};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register).service(login).service(logout);
}
