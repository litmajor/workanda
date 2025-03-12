// src/user_management/profile/freelancer.rs

use sqlx::PgPool;

pub async fn create_freelancer_profile(
    pool: &PgPool,
    user_id: i32,
    specialization: &str,
    hourly_rate: f64,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        INSERT INTO freelancer_profiles (user_id, specialization, hourly_rate)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        specialization,
        hourly_rate
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

use sqlx::PgPool;

pub struct FreelancerProfile {
    pub user_id: i32,
    pub specialization: String,
    pub portfolio_url: Option<String>,
    pub hourly_rate: f64,
    pub kyc_status: bool,
}

impl FreelancerProfile {
    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        data: FreelancerData
    ) -> Result<FreelancerProfile, sqlx::Error> {
        sqlx::query_as!(
            FreelancerProfile,
            r#"
            INSERT INTO freelancer_profiles (user_id, specialization, portfolio_url, hourly_rate)
            VALUES ($1, $2, $3, $4)
            RETURNING user_id, specialization, portfolio_url, hourly_rate, kyc_status
            "#,
            user_id,
            data.specialization,
            data.portfolio_url,
            data.hourly_rate
        )
        .fetch_one(pool)
        .await
    }
}