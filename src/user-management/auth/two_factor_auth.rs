
use sqlx::PgPool;

pub async fn enable_2fa(
    pool: &PgPool,
    user_id: i32,
    secret: &str,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        UPDATE users
        SET two_factor_secret = $1, two_factor_enabled = TRUE
        WHERE id = $2
        "#,
        secret,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn disable_two_factor_auth(
    pool: &PgPool,
    user_id: i32,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        UPDATE two_factor_auth
        SET enabled = FALSE, secret = NULL, recovery_codes = NULL
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}