// src/admin/dispute.rs
use crate::models::{User, UserRole};
use crate::escrow::resolve_dispute;
use sqlx::PgPool;

pub async fn admin_resolve_dispute(
    pool: &PgPool,
    current_user: &User,
    contract_id: u32,
    resolution: DisputeResolution,
) -> Result<(), String> {
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    resolve_dispute(pool, contract_id, resolution).await
}


pub async fn escalate_dispute(
    pool: &PgPool,
    current_user: &User,
    dispute_id: i32,
) -> Result<(), String> {
    // Access Control: Ensure only Admins can escalate disputes
    if current_user.role != UserRole::Admin {
        return Err("Unauthorized".to_string());
    }

    // Fetch the dispute details to ensure it exists and is in a valid state for escalation
    let dispute = sqlx::query_as!(
        Dispute,
        r#"
        SELECT id, contract_id, level, status
        FROM disputes
        WHERE id = $1 AND status = 'PENDING'
        "#,
        dispute_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| "Dispute not found or not pending.".to_string())?;

    // Determine the next escalation level
    let next_level = match dispute.level {
        DisputeLevel::InitialReview => DisputeLevel::Mediation,
        DisputeLevel::Mediation => DisputeLevel::Arbitration,
        DisputeLevel::Arbitration => DisputeLevel::Resolved,
        DisputeLevel::Resolved => return Err("Dispute is already resolved.".to_string()),
    };

    // Update the dispute level in the database
    sqlx::query!(
        r#"
        UPDATE disputes
        SET level = $1, updated_at = NOW()
        WHERE id = $2
        "#,
        next_level as _,
        dispute_id
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    // Log the admin action in activity_logs (Audit Trail)
    log_admin_action(
        pool,
        current_user.id,
        "ESCALATE_DISPUTE",
        Some(format!("Dispute ID {} escalated to {:?}", dispute_id, next_level)),
    )
    .await?;

    Ok(())
}