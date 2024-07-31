use actix_web::web;

use crate::auth::api::auth::{create_user, get_users, login, logout};

pub fn auth_open_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(login).service(logout);
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
}
