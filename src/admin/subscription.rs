pub async fn update_subscription(
    pool: &PgPool,
    current_user: &User,
    user_id: i32,
    new_status: SubscriptionStatus,
) -> Result<User, String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET subscription_status = $1
        WHERE id = $2
        RETURNING id, username, email, subscription_status
        "#,
        new_status as _,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}