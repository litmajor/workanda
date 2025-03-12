// src/admin/membership.rs
use crate::models::MembershipTier;
use sqlx::PgPool;

pub async fn create_membership_tier(
    pool: &PgPool,
    tier: &MembershipTier,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        INSERT INTO membership_tiers (name, price, features, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        tier.name,
        tier.price,
        tier.features.as_ref(),
        Utc::now(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn update_membership_tier(
    pool: &PgPool,
    tier_id: i32,
    updated_tier: &MembershipTier,
) -> Result<(), String> {
    sqlx::query!(
        r#"
        UPDATE membership_tiers
        SET name = $1, price = $2, features = $3, updated_at = $4
        WHERE id = $5
        "#,
        updated_tier.name,
        updated_tier.price,
        updated_tier.features.as_ref(),
        Utc::now(),
        tier_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn delete_membership_tier(
    pool: &PgPool,
    tier_id: i32,
) -> Result<(), String> {
    sqlx::query!(
        "DELETE FROM membership_tiers WHERE id = $1",
        tier_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}