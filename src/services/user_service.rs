use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

use crate::models::user::User;
use crate::repositories::user_repository;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(serde::Serialize)]
pub struct PaginatedUsers {
    pub data: Vec<User>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

pub fn get_users_paginated(
    pool: &DbPool,
    page: i64,
    per_page: i64,
) -> Result<PaginatedUsers, String> {
    let mut conn = pool.get().map_err(|e| format!("Erro de conexão: {}", e))?;

    let (users, total) = user_repository::find_all_paginated(&mut conn, page, per_page)
        .map_err(|e| format!("Erro na consulta: {}", e))?;

    let total_pages = if total == 0 {
        0
    } else {
        (total as f64 / per_page as f64).ceil() as i64
    };

    Ok(PaginatedUsers {
        data: users,
        page,
        per_page,
        total,
        total_pages,
    })
}
