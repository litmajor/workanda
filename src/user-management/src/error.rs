use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Authentication required")]
    Unauthorized,
    
    #[error("Invalid token")]
    InvalidToken,
    
    #[error("Token creation failed")]
    TokenCreation,
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Argon2 error: {0}")]
    Argon2(#[from] argon2::password_hash::Error),
    
    #[error("System time error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),
}