pub async fn submit_verification_request(
    pool: &PgPool,
    user_id: i32,
    documents: Vec<String>, // List of document URLs or IDs
) -> Result<(), String> {
    sqlx::query!(
        r#"
        INSERT INTO verification_requests (user_id, documents, status)
        VALUES ($1, $2, 'SUBMITTED')
        "#,
        user_id,
        documents.as_slice()
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn update_verification_status(
    pool: &PgPool,
    user_id: i32,
    status: &str, // "VERIFIED" or "REJECTED"
) -> Result<(), String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    // Update the user's verification status
    sqlx::query!(
        r#"
        UPDATE users
        SET verification_status = $1
        WHERE id = $2
        "#,
        status,
        user_id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?;

    // Update the verification request status
    sqlx::query!(
        r#"
        UPDATE verification_requests
        SET status = $1
        WHERE user_id = $2
        "#,
        status,
        user_id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?;

    transaction.commit().await.map_err(|e| e.to_string())
}