use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use futures::future::LocalBoxFuture;
use log::error;


pub struct AuthMiddlewareFactory;

impl<S> actix_web::middleware::Middleware<S> for AuthMiddlewareFactory {
    fn start(&self, req: ServiceRequest) -> LocalBoxFuture<'static, Result<ServiceRequest, Error>> {
        let token = req.headers()
            .get("Authorization")
            .and_then(|auth| auth.to_str().ok())
            .map(|s| s.strip_prefix("Bearer ").unwrap_or_default())
            .unwrap_or_default();

        match decode_jwt(token) {
            Ok(claims) => {
                if claims.role == "Admin" && req.path().starts_with("/v1/admin") {
                    Box::pin(async { Ok(req) })
                } else if claims.role == "Client" && req.path().starts_with("/v1/client") {
                    Box::pin(async { Ok(req) })
                } else if claims.role == "Freelancer" && req.path().starts_with("/v1/freelancer") {
                    Box::pin(async { Ok(req) })
                } else {
                    error!("Unauthorized access attempt to {}", req.path());
                    Box::pin(async {
                        Err(actix_web::error::ErrorUnauthorized("Unauthorized").into())
                    })
                }
            }
            Err(_) => {
                error!("Invalid JWT token provided");
                Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid token").into())
                })
            }
        }
    }
}

fn decode_jwt(token: &str) -> Result<JwtClaims, ApiError> {
    let secret_key = std::env::var("JWT_SECRET_KEY").map_err(|_| ApiError::InternalServerError)?;
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret_key.as_bytes());
    let validation = jsonwebtoken::Validation::default();

    jsonwebtoken::decode::<JwtClaims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|_| ApiError::Unauthorized("Invalid token".to_string()))
}

#[derive(Debug, Deserialize)]
pub struct JwtClaims {
    pub sub: String, // User ID
    pub role: String, // User role (e.g., "Admin", "Client", "Freelancer")
}