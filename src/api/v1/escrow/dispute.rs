use sqlx::{PgPool, Error};
use crate::models::DisputeResolution;
use crate::services::escrow_service::handle_dispute;


#[derive(Deserialize)]
pub struct DisputeRequest {
    pub escrow_id: u32,
    pub resolution: DisputeResolution,
}


pub async fn handle_dispute(
    pool: &PgPool,
    escrow_id: u32,
    resolution: DisputeResolution,
) -> Result<(), String> {
    match resolution {
        DisputeResolution::Refund => refund_escrow_funds(pool, escrow_id).await.map_err(|e| e.to_string()),
        DisputeResolution::Release => release_escrow_funds(pool, escrow_id).await.map_err(|e| e.to_string()),
        _ => Err("Unsupported dispute resolution".to_string())
    }
}

async fn refund_escrow_funds(pool: &PgPool, escrow_id: u32) -> Result<(), String> {
    // Actual refund logic
    Ok(())
}

async fn release_escrow_funds(pool: &PgPool, escrow_id: u32) -> Result<(), String> {
    // Actual release logic
    Ok(())
}
