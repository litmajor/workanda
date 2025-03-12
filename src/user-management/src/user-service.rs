// src/user_management/user.rs

use crate::models::User;
use sqlx::PgPool;
use argon2::{Argon2, PasswordHasher};
use rand::RngCore;


pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: String,
    role_id: i32,
) -> Result<User, String> {
    // Create a new user in the database
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, role_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, password_hash, role_id
        "#,
        username,
        email,
        password_hash,
        role_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<Option<User>, String> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(user)
}

fn hash_password(input_password: &str) -> Result<String, String> {
    // Generate a random salt
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);

    // Create an Argon2 instance
    let argon2 = Argon2::default();

    // Hash the password
    argon2.hash_password(input_password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Failed to hash password: {}", e))
}