// workanda-auth/src/lib.rs
use std::sync::Arc;
use parking_lot::RwLock;
use redis::Client as RedisClient;

pub struct AuthManager {
    secret_key: String,
    redis: Arc<RwLock<RedisClient>>,
}

impl AuthManager {
    pub fn new(secret_key: &str, redis_uri: &str) -> Self {
        let redis = RedisClient::open(redis_uri).expect("Failed to connect to Redis");
        Self {
            secret_key: secret_key.to_string(),
            redis: Arc::new(RwLock::new(redis)),
        }
    }

    pub async fn generate_tokens(&self, user_id: &str, role: &str) -> Result<(String, String), String> {
        let access_token = self.create_token(user_id, role, 3600).await?; // 1 hour
        let refresh_token = self.create_token(user_id, role, 2592000).await?; // 30 days

        // Store refresh token in Redis with TTL
        let mut conn = self.redis.write().get_async_connection().await.map_err(|e| e.to_string())?;
        redis::cmd("SETEX")
            .arg(format!("refresh_token:{}", user_id))
            .arg(2592000)
            .arg(&refresh_token)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok((access_token, refresh_token))
    }

    async fn create_token(&self, user_id: &str, role: &str, ttl_secs: u64) -> Result<String, String> {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        PasetoBuilder::new()
            .set_claim(Claim::Subject(user_id.to_string()))
            .set_claim(Claim::IssuedAt(current_time.into()))
            .set_claim(Claim::Expiration((current_time + ttl_secs).into()))
            .set_footer(json!({ "role": role }).to_string())
            .build(&self.secret_key)
            .map_err(|e| format!("Token generation failed: {}", e))
    }
}