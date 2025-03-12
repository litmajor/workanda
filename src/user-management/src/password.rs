pub mod password {
    use argon2::{Argon2, PasswordHash, PasswordHasher};
    use chrono::Utc;
    use rand::{thread_rng, Rng};
    use sqlx::PgPool;
    use uuid::Uuid;

    pub async fn generate_reset_token(
        pool: &PgPool,
        user_id: i32
    ) -> Result<String, sqlx::Error> {
        let token: String = thread_rng().gen::<u32>().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(1);

        sqlx::query!(
            "INSERT INTO password_reset_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)",
            user_id,
            token,
            expires_at
        )
        .execute(pool)
        .await?;

        Ok(token)
    }

    pub async fn reset_password(
        pool: &PgPool,
        token: &str,
        new_password: &str
    ) -> Result<(), sqlx::Error> {
        let reset_token = sqlx::query_as!(
            ResetToken,
            "SELECT * FROM password_reset_tokens WHERE token = $1 AND expires_at > NOW()",
            token
        )
        .fetch_one(pool)
        .await?;

        let argon2 = Argon2::default();
        let salt = Uuid::new_v4().as_bytes().to_vec();
        let password_hash = argon2.hash_password(new_password.as_bytes(), &salt)?
            .to_string();

        sqlx::query!(
            "UPDATE users SET password_hash = $1 WHERE id = $2",
            password_hash,
            reset_token.user_id
        )
        .execute(pool)
        .await?;

        sqlx::query!(
            "DELETE FROM password_reset_tokens WHERE token = $1",
            token
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
