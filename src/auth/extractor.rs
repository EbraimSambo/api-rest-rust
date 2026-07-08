use actix_web::{web, FromRequest, HttpRequest};
use uuid::Uuid;

use super::jwt::verify_token;
use super::models::JwtSecret;

pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let jwt_secret = match req.app_data::<web::Data<JwtSecret>>() {
            Some(data) => &data.0,
            None => {
                return std::future::ready(Err(actix_web::error::ErrorInternalServerError(
                    "JWT não configurado",
                )))
            }
        };

        let auth_header = match req.headers().get("Authorization") {
            Some(val) => val,
            None => {
                return std::future::ready(Err(actix_web::error::ErrorUnauthorized(
                    "Token não fornecido",
                )))
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return std::future::ready(Err(actix_web::error::ErrorUnauthorized(
                    "Header inválido",
                )))
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return std::future::ready(Err(actix_web::error::ErrorUnauthorized(
                "Formato inválido. Use: Bearer <token>",
            )));
        }

        let token = &auth_str[7..];

        match verify_token(token, jwt_secret) {
            Ok(claims) => std::future::ready(Ok(AuthenticatedUser {
                id: claims.sub,
                email: claims.email,
            })),
            Err(e) => std::future::ready(Err(actix_web::error::ErrorUnauthorized(e))),
        }
    }
}
