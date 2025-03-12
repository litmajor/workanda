// src/admin/notifications.rs
use crate::models::{SystemNotification, UserNotification};
use sqlx::PgPool;

pub async fn create_system_notification(
    pool: &PgPool,
    notification: &SystemNotification,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        INSERT INTO system_notifications (message, notification_type, created_at)
        VALUES ($1, $2, $3)
        "#,
        notification.message,
        notification.notification_type,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn mark_notification_read(
    pool: &PgPool,
    notification_id: i32,
) -> Result<(), String> {
    sqlx::query!(
        "UPDATE system_notifications SET is_read = TRUE WHERE id = $1",
        notification_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_user_notifications(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<UserNotification>, String> {
    let notifications = sqlx::query_as!(
        UserNotification,
        r#"
        SELECT id, user_id, message, notification_type, is_read, created_at
        FROM user_notifications
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 100
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(notifications)
}