use actix_web::web;
use crate::routes::routes::{home, list_users};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home)
       .service(list_users);
}