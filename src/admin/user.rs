// src/admin/user.rs
use crate::models::{User, UserRole};
use crate::user_management::user::{create_user, update_user_role};
use sqlx::PgPool;

pub async fn create_admin_user(
    pool: &PgPool,
    current_user: &User,
    username: &str,
    email: &str,
    password: &str,
    role: UserRole,
    membership_tier: MembershipTier,
    location: Location,
) -> Result<User, String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    let hashed_password = hash_password(password).await?;
    create_user(
        pool,
        username,
        email,
        &hashed_password,
        role,
        membership_tier,
        location,
    )
    .await
}

pub async fn delete_admin_user(
    pool: &PgPool,
    current_user: &User,
    user_id: i32,
) -> Result<(), String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

