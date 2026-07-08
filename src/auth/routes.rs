use actix_web::{get, post, web, HttpResponse, Responder};

use crate::auth::extractor::AuthenticatedUser;
use crate::auth::models::{JwtSecret, LoginRequest};
use crate::auth::service;
use crate::models::user::UserResponse;
use crate::services::user_service::DbPool;

#[post("/auth/login")]
async fn login(
    pool: web::Data<DbPool>,
    jwt_secret: web::Data<JwtSecret>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    match service::login(pool.get_ref(), &jwt_secret.0, body.into_inner()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({"error": e})),
    }
}

#[get("/auth/me")]
async fn me(auth: AuthenticatedUser, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": "Erro de conexão"})),
    };

    match crate::repositories::user_repository::find_by_id(&mut conn, auth.id) {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({"error": "Usuário não encontrado"})),
    }
}

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(me);
}
