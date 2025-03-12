// src/user_management/auth/login.rs

use crate::models::User;
use sqlx::PgPool;
use argon2::{Argon2, PasswordVerifier};
use std::str;

pub async fn login(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<User, String> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        username
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Verify the password
    if verify_password(password, &user.password_hash)? {
        Ok(user)
    } else {
        Err("Invalid password".to_string())
    }
}

fn verify_password(input_password: &str, hashed_password: &str) -> Result<bool, String> {
    // Parse the hashed password into a PasswordHash struct
    let password_hash = argon2::PasswordHash::new(hashed_password)
        .map_err(|e| format!("Failed to parse password hash: {}", e))?;

    // Create an Argon2 instance
    let verifier = Argon2::default();

    // Verify the input password against the stored hash
    verifier
        .verify_password(input_password.as_bytes(), &password_hash)
        .map(|_| true) // Return true if verification succeeds
        .map_err(|e| format!("Password verification failed: {}", e))
}

pub async fn login_with_2fa(
    pool: &PgPool,
    username: &str,
    password: &str,
    totp_code: Option<&str>,
) -> Result<User, String> {
    // Step 1: Authenticate the user with their password
    let user = login(pool, username, password).await?;

    // Step 2: Check if 2FA is enabled for the user
    let two_factor = sqlx::query_as!(
        TwoFactorAuth,
        r#"
        SELECT id, secret, enabled FROM two_factor_auth WHERE user_id = $1
        "#,
        user.id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(tfa) = two_factor {
        if tfa.enabled {
            // Step 3: Verify the TOTP code or recovery code
            if let Some(code) = totp_code {
                if verify_totp_code(pool, user.id, code).await? {
                    return Ok(user);
                } else if use_recovery_code(pool, user.id, code).await? {
                    return Ok(user);
                } else {
                    return Err("Invalid TOTP code or recovery code.".to_string());
                }
            } else {
                return Err("2FA is enabled. Please provide a TOTP code or recovery code.".to_string());
            }
        }
    }

    Ok(user)
}

