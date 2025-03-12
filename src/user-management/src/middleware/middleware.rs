use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use paseto::tokens::validate_local_token;
use redis::Client as RedisClient;
use uuid::Uuid;

pub struct AuthenticatedUser {
    pub id: Uuid,
    pub role: String,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret_key = req.app_data::<String>().expect("Secret key not configured");
    let redis = req.app_data::<RedisClient>().expect("Redis client not configured");

    let token = credentials.token();
    
    // Validate token structure
    let claims = validate_local_token(
        token,
        None,
        &secret_key,
        &paseto::tokens::TimeBackend::Chrono
    ).map_err(|e| (Error::from(e), req))?;

    let user_id = Uuid::parse_str(claims["sub"].as_str().unwrap())
        .map_err(|e| (Error::from(e), req))?;

    // Check Redis for token validity
    let mut conn = redis.get_async_connection()
        .await
        .map_err(|e| (Error::from(e), req))?;
    
    let exists: bool = redis::cmd("EXISTS")
        .arg(format!("refresh_token:{}", user_id))
        .query_async(&mut conn)
        .await
        .map_err(|e| (Error::from(e), req))?;

    if !exists {
        return Err((Error::from(AuthError::InvalidToken), req));
    }

    // Attach user to request
    req.extensions_mut().insert(AuthenticatedUser {
        id: user_id,
        role: claims["role"].as_str().unwrap().to_string(),
    });

    Ok(req)
}