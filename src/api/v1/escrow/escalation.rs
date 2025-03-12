use sqlx::{PgPool, Error};
use crate::models::DisputeLevel;
use crate::services::escrow_service::escalate_dispute;



#[derive(Deserialize)]
pub struct EscalationRequest {
    pub dispute_id: u32,
}


pub async fn escalate_dispute(
    pool: &PgPool,
    dispute_id: u32,
) -> Result<(), Error> {
    use crate::schema::disputes::dsl::*;

    let current_level: DisputeLevel = sqlx::query!(
        r#"
        SELECT level FROM disputes WHERE id = $1
        "#,
        dispute_id
    )
    .fetch_one(pool)
    .await?
    .level;

    let next_level = match current_level {
        DisputeLevel::InitialReview => DisputeLevel::Mediation,
        DisputeLevel::Mediation => DisputeLevel::Arbitration,
        DisputeLevel::Arbitration => DisputeLevel::Resolved,
        _ => return Err(Error::RowNotFound),
    };

    sqlx::query!(
        r#"
        UPDATE disputes SET level = $1 WHERE id = $2
        "#,
        next_level as i32,
        dispute_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
