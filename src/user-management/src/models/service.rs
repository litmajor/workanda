use crate::user::{model::User, error::AuthError};
use sqlx::PgPool;
use paseto::tokens::PasetoBuilder;
use std::time::{SystemTime, Duration};
use redis::Client as RedisClient;

pub struct AuthService {
    db_pool: PgPool,
    redis: RedisClient,
    secret_key: String,
}

impl AuthService {
    pub fn new(db_pool: PgPool, redis: RedisClient, secret_key: String) -> Self {
        Self { db_pool, redis, secret_key }
    }

    pub async fn register(&self, new_user: &NewUser) -> Result<UserResponse, AuthError> {
        let user = User::new(new_user.email.clone(), new_user.password.clone())?;
        
        sqlx::query!(
            r#"INSERT INTO users (id, email, password_hash, role, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6)"#,
            user.id,
            user.email,
            user.password_hash,
            user.role,
            user.created_at,
            user.updated_at
        )
        .execute(&self.db_pool)
        .await?;

        Ok(UserResponse::from(user))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(String, String), AuthError> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.db_pool)
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

        if !user.verify_password(password) {
            return Err(AuthError::InvalidCredentials);
        }

        // Generate tokens
        let access_token = self.generate_token(&user.id, 3600).await?; // 1 hour
        let refresh_token = self.generate_token(&user.id, 2592000).await?; // 30 days

        // Store refresh token in Redis
        let mut conn = self.redis.get_async_connection().await?;
        redis::cmd("SETEX")
            .arg(format!("refresh_token:{}", user.id))
            .arg(2592000)
            .arg(&refresh_token)
            .query_async(&mut conn)
            .await?;

        Ok((access_token, refresh_token))
    }

    async fn generate_token(&self, user_id: &Uuid, ttl_secs: u64) -> Result<String, AuthError> {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        PasetoBuilder::new()
            .set_claim("sub", user_id.to_string())
            .set_claim("iat", current_time)
            .set_claim("exp", current_time + ttl_secs)
            .build(&self.secret_key)
            .map_err(|_| AuthError::TokenCreation)
    }
}