use oath::totp;
use rand::Rng;
use base32;

pub async fn enable_two_factor_auth(
    pool: &PgPool,
    user_id: i32,
) -> Result<(String, Vec<String>), String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    // Generate a random TOTP secret
    let secret_key: [u8; 20] = rand::thread_rng().gen(); // 20-byte random secret
    let base32_secret = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &secret_key);

    // Generate recovery codes
    let recovery_codes: Vec<String> = (0..10)
        .map(|_| {
            let code: [u8; 6] = rand::thread_rng().gen();
            base32::encode(base32::Alphabet::RFC4648 { padding: false }, &code)
        })
        .collect();

    // Hash recovery codes for secure storage
    let hashed_recovery_codes: Vec<String> = recovery_codes
        .iter()
        .map(|code| hash_recovery_code(code))
        .collect();

    // Insert or update 2FA details for the user
    sqlx::query!(
        r#"
        INSERT INTO two_factor_auth (user_id, secret, recovery_codes, enabled)
        VALUES ($1, $2, $3, TRUE)
        ON CONFLICT (user_id) DO UPDATE
        SET secret = $2, recovery_codes = $3, enabled = TRUE, updated_at = NOW()
        "#,
        user_id,
        base32_secret,
        &hashed_recovery_codes
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?;

    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok((base32_secret, recovery_codes))
}

fn hash_recovery_code(code: &str) -> String {
    use argon2::{Argon2, PasswordHasher};
    let argon2 = Argon2::default();
    let salt = rand::thread_rng().gen::<[u8; 16]>();
    argon2.hash_password(code.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .unwrap_or_else(|_| "INVALID".to_string())
}

pub async fn verify_totp_code(
    pool: &PgPool,
    user_id: i32,
    code: &str,
) -> Result<bool, String> {
    let two_factor = sqlx::query_as!(
        TwoFactorAuth,
        r#"
        SELECT id, secret FROM two_factor_auth WHERE user_id = $1 AND enabled = TRUE
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(tfa) = two_factor {
        let secret = tfa.secret;
        let valid = totp::verify_totp_default(code, &secret).is_ok();
        Ok(valid)
    } else {
        Err("2FA is not enabled for this user.".to_string())
    }
}

pub async fn use_recovery_code(
    pool: &PgPool,
    user_id: i32,
    code: &str,
) -> Result<bool, String> {
    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    let two_factor = sqlx::query_as!(
        TwoFactorAuth,
        r#"
        SELECT id, recovery_codes FROM two_factor_auth WHERE user_id = $1 AND enabled = TRUE
        "#,
        user_id
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(recovery_codes) = two_factor.recovery_codes {
        let hashed_code = hash_recovery_code(code);
        if recovery_codes.contains(&hashed_code) {
            // Remove the used recovery code
            let updated_codes: Vec<String> = recovery_codes
                .into_iter()
                .filter(|c| c != &hashed_code)
                .collect();

            sqlx::query!(
                r#"
                UPDATE two_factor_auth
                SET recovery_codes = $1
                WHERE user_id = $2
                "#,
                &updated_codes,
                user_id
            )
            .execute(&mut *transaction)
            .await
            .map_err(|e| e.to_string())?;

            transaction.commit().await.map_err(|e| e.to_string())?;
            return Ok(true);
        }
    }

    Err("Invalid recovery code.".to_string())
}