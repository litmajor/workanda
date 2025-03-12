use actix_web::{web, Scope};
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(handlers::login))
            .route("/register", web::post().to(handlers::register)),
    );
}