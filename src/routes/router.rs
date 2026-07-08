use actix_web::web;
use crate::auth::routes::configure_auth_routes;
use crate::routes::routes::{create_user, home, list_users};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home)
       .service(list_users)
       .service(create_user)
       .configure(configure_auth_routes);
}