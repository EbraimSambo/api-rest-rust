use actix_web::web;
use crate::routes::routes::home;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
}