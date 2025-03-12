// src/admin/log.rs
use sqlx::PgTransaction;
use chrono::Utc;

pub async fn log_admin_action(
    transaction: &mut PgTransaction<'_>,
    admin_id: i32,
    action: &str,
    details: Option<String>,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        INSERT INTO admin_activity_logs (admin_id, action, details, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        admin_id,
        action,
        details,
        Utc::now()
    )
    .execute(transaction)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}