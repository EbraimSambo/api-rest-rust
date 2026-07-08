use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rand::rngs::OsRng;

use crate::models::user::{CreateUserRequest, NewUser, User, UserResponse};
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

#[derive(serde::Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub enum CreateUserError {
    Validation(Vec<ValidationError>),
    Internal(String),
}

fn validate_input(req: &CreateUserRequest) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    let name = req.name.trim();
    if name.is_empty() {
        errors.push(ValidationError {
            field: "name".into(),
            message: "Nome é obrigatório".into(),
        });
    } else if name.len() > 100 {
        errors.push(ValidationError {
            field: "name".into(),
            message: "Nome deve ter no máximo 100 caracteres".into(),
        });
    }

    let email = req.email.trim();
    if email.is_empty() {
        errors.push(ValidationError {
            field: "email".into(),
            message: "Email é obrigatório".into(),
        });
    } else if !email.contains('@') || !email.contains('.') {
        errors.push(ValidationError {
            field: "email".into(),
            message: "Email inválido".into(),
        });
    }

    if req.password.len() < 6 {
        errors.push(ValidationError {
            field: "password".into(),
            message: "Senha deve ter no mínimo 6 caracteres".into(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn create_user(
    pool: &DbPool,
    req: CreateUserRequest,
) -> Result<UserResponse, CreateUserError> {
    validate_input(&req).map_err(CreateUserError::Validation)?;

    let mut conn = pool
        .get()
        .map_err(|e| CreateUserError::Internal(format!("Erro de conexão: {}", e)))?;

    if user_repository::find_by_email(&mut conn, &req.email.trim().to_lowercase()).is_ok() {
        return Err(CreateUserError::Validation(vec![ValidationError {
            field: "email".into(),
            message: "Email já cadastrado".into(),
        }]));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|e| CreateUserError::Internal(format!("Erro ao gerar hash: {}", e)))?
        .to_string();

    let new_user = NewUser {
        name: req.name.trim().to_string(),
        email: req.email.trim().to_string(),
        password: password_hash,
        created_at: Utc::now(),
    };

    let user = user_repository::insert(&mut conn, &new_user)
        .map_err(|e| CreateUserError::Internal(format!("Erro ao inserir usuário: {}", e)))?;

    Ok(UserResponse::from(user))
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
