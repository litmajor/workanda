pub async fn set_platform_fee(
    pool: &PgPool,
    current_user: &User,
    new_fee: f64,
) -> Result<(), String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    sqlx::query!(
        r#"
        UPDATE settings
        SET platform_fee = $1
        WHERE id = 1 -- Assuming a singleton settings row
        "#,
        new_fee
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}