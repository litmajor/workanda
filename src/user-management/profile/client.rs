// src/user_management/profile/client.rs

use sqlx::PgPool;

pub async fn create_client_profile(
    pool: &PgPool,
    user_id: i32,
    company_name: &str,
    tax_id: Option<&str>,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        INSERT INTO client_profiles (user_id, company_name, tax_id)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        company_name,
        tax_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

use sqlx::PgPool;

pub struct ClientProfile {
    pub user_id: i32,
    pub company_name: String,
    pub tax_id: Option<String>,
    pub kyc_status: bool,
    pub payment_methods: Vec<String>,
}

impl ClientProfile {
    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        data: ClientData
    ) -> Result<ClientProfile, sqlx::Error> {
        sqlx::query_as!(
            ClientProfile,
            r#"
            INSERT INTO client_profiles (user_id, company_name, tax_id)
            VALUES ($1, $2, $3)
            RETURNING user_id, company_name, tax_id, kyc_status, payment_methods
            "#,
            user_id,
            data.company_name,
            data.tax_id
        )
        .fetch_one(pool)
        .await
    }
}