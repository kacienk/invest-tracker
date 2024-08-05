use actix_web::web;

use super::views::{
    create_investment_group, delete_investment_group, get_all_investment_groups,
    get_investment_group, update_investment_group,
};

pub fn investment_groups_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_investment_groups)
        .service(get_investment_group)
        .service(create_investment_group)
        .service(update_investment_group)
        .service(delete_investment_group);
}
