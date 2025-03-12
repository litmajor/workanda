// src/middleware/auth.rs
use actix_web::{dev::ServiceRequest, Error};
use futures::future::{ok, Ready};

pub fn authorize_role(role: UserRole) -> impl Fn(ServiceRequest) -> Ready<Result<ServiceRequest, Error>> {
    move |req| {
        if let Some(claims) = req.extensions().get::<Claims>() {
            if claims.role == role {
                return ok(req);
            }
        }
        ok(req.error_response(actix_web::http::StatusCode::FORBIDDEN))
    }
}