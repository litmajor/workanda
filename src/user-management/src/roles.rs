#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn create_role(
    pool: &PgPool,
    role_name: &str,
    description: Option<String>,
) -> Result<Role, String> {
    let result = sqlx::query_as!(
        Role,
        r#"
        INSERT INTO roles (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at, updated_at
        "#,
        role_name,
        description
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result)
}

pub async fn get_role_by_name(
    pool: &PgPool,
    role_name: &str,
) -> Result<Option<Role>, String> {
    let role = sqlx::query_as!(
        Role,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM roles
        WHERE name = $1
        "#,
        role_name
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(role)
}
