use argon2::password_hash::PasswordHash;
use argon2::PasswordVerifier;
use argon2::Argon2;

use crate::auth::jwt::create_token;
use crate::auth::models::{LoginRequest, LoginResponse};
use crate::repositories::user_repository;
use crate::services::user_service::DbPool;

pub fn login(pool: &DbPool, jwt_secret: &str, req: LoginRequest) -> Result<LoginResponse, String> {
    let mut conn = pool.get().map_err(|e| format!("Erro de conexão: {}", e))?;

    let user = user_repository::find_by_email(&mut conn, &req.email)
        .map_err(|_| "Email ou senha inválidos".to_string())?;

    let parsed_hash =
        PasswordHash::new(&user.password).map_err(|_| "Erro interno".to_string())?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| "Email ou senha inválidos".to_string())?;

    let token = create_token(user.id, &user.email, jwt_secret)?;

    Ok(LoginResponse {
        token,
        user_id: user.id,
        email: user.email,
    })
}
