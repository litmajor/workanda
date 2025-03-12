use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use chrono::Utc;

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    hash(password, DEFAULT_COST).map_err(|_| ApiError::InternalServerError)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

pub fn create_jwt(user: &User) -> Result<String, ApiError> {
    let claims = JwtClaims {
        sub: user.id.to_string(),
        role: user.role.to_string(),
        exp: (Utc::now() + chrono::Duration::days(7)).timestamp() as usize, // Token expires in 7 days
    };

    let secret_key = std::env::var("JWT_SECRET_KEY").map_err(|_| ApiError::InternalServerError)?;
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());

    encode(&Header::new(Algorithm::HS256), &claims, &encoding_key)
        .map_err(|_| ApiError::InternalServerError)
}

#[derive(Debug, Serialize)]
struct JwtClaims {
    pub sub: String, // Subject (user ID)
    pub role: String, // User role (e.g., "Client", "Freelancer")
    pub exp: usize, // Expiration time (as Unix timestamp)
}

use uuid::Uuid;

pub async fn ensure_account_exists(
    pool: &PgPool,
    user_id: Uuid,
    role: &str,
) -> Result<i32, String> {
    // Check if the account already exists
    let account = sqlx::query!(
        r#"
        SELECT id FROM accounts WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(row) = account {
        return Ok(row.id); // Return the existing account ID
    }

    // Create a new account if it doesn't exist
    let result = sqlx::query!(
        r#"
        INSERT INTO accounts (user_id, role)
        VALUES ($1, $2)
        RETURNING id
        "#,
        user_id,
        role
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.id) // Return the newly created account ID
}

async fn initialize_platform_account(pool: &PgPool) -> Result<(), String> {
    let platform_user_id = Uuid::from_u128(0); // Reserved UUID for the platform
    ensure_account_exists(pool, platform_user_id, "PLATFORM").await?;
    Ok(())
}

