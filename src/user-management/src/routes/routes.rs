use actix_web::{web, HttpResponse};
use crate::user::{model::{NewUser, UserResponse}, service::AuthService, error::AuthError};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/refresh", web::post().to(refresh_token))
            .route("/logout", web::post().to(logout))
    );
}

async fn register(
    service: web::Data<AuthService>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, AuthError> {
    let user_response = service.register(&new_user).await?;
    Ok(HttpResponse::Created().json(user_response))
}

async fn login(
    service: web::Data<AuthService>,
    credentials: web::Json<NewUser>,
) -> Result<HttpResponse, AuthError> {
    let (access_token, refresh_token) = service.login(&credentials.email, &credentials.password).await?;
    Ok(HttpResponse::Ok().json(json!({
        "access_token": access_token,
        "refresh_token": refresh_token
    })))
}

// Implement refresh_token and logout endpoints similarly