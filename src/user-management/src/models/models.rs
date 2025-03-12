use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use derive_more::{Display, FromStr};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Postgres, QueryAs};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub mfa_enabled: bool,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub kyc_status: bool,
    pub two_factor_enabled: bool, // Indicates if 2FA is enabled
    pub two_factor_secret: Option<String>, // TOTP secret key
    pub recovery_codes: Option<Vec<String>>, // Hashed recovery codes
}


#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password: String) -> Result<Self, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
        
        Ok(Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            role: "user".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.password_hash).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub provider: String,
    pub provider_id: String,
    pub access_token: String, // Encrypted
    pub refresh_token: Option<String>, // Encrypted
}


#[derive(Debug, FromRow)]
pub struct ResetToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}