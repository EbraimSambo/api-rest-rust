use actix_web::web;
use crate::routes::routes::hello;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}