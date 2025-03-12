// src/admin/role.rs
use crate::models::{Role, UserRole};
use crate::user_management::role::create_role;
use sqlx::PgPool;

pub async fn create_admin_role(
    pool: &PgPool,
    current_user: &User,
    name: &str,
    description: Option<String>,
) -> Result<Role, String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    create_role(pool, name, description).await
}

pub async fn update_admin_role(
    pool: &PgPool,
    current_user: &User,
    role_id: i32,
    new_name: &str,
    new_description: Option<String>,
) -> Result<Role, String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    sqlx::query_as!(
        Role,
        r#"
        UPDATE roles
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description
        "#,
        new_name,
        new_description,
        role_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}