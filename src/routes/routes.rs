use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::user_service::{self, DbPool};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().json("Hello Actix Web!")
}

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    per_page: Option<i64>,
}

#[get("/users")]
async fn list_users(
    pool: web::Data<DbPool>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1, 100);

    match user_service::get_users_paginated(pool.get_ref(), page, per_page) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": e})),
    }
}
