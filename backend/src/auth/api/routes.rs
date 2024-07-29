use actix_web::web;

use crate::auth::api::auth::{create_user, login, logout};

pub fn auth_open_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(login).service(logout);
}

pub fn auth_protected_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(get_user).service(update_user).service(delete_user);
}
