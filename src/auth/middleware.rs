use jsonwebtoken::{decode, Validation, DecodingKey, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use futures::future::LocalBoxFuture;


#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String, // Subject (user ID or username)
    pub role: String, // User role (e.g., "Admin", "Client", "Freelancer")
    pub exp: usize, // Expiration time (as Unix timestamp)
}

pub fn decode_jwt(token: &str) -> Result<JwtClaims, AuthError> {
    let secret_key = std::env::var("JWT_SECRET_KEY").map_err(|_| AuthError::MissingToken)?;
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());

    let validation = Validation {
        validate_exp: true,
        ..Default::default()
    };

    match decode::<JwtClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => {
            if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = err.kind() {
                return Err(AuthError::TokenExpired);
            }
            Err(AuthError::InvalidToken(err))
        }
    }
}

pub struct AuthMiddlewareFactory;

impl<S> actix_web::middleware::Middleware<S> for AuthMiddlewareFactory {
    fn start(&self, req: ServiceRequest) -> LocalBoxFuture<'static, Result<ServiceRequest, Error>> {
        let token = req.headers()
            .get("Authorization")
            .and_then(|auth| auth.to_str().ok())
            .map(|s| s.strip_prefix("Bearer ").unwrap_or_default())
            .unwrap_or_default();

        match decode_jwt(token) {
            Ok(claims) if claims.role == "Admin" => Box::pin(async { Ok(req) }), // Allow access if admin
            _ => Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized").into()) }),
        }
    }
}