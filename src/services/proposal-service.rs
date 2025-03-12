// services/proposal_service.rs
use crate::models::{Proposal, ProposalStatus};
use sqlx::PgPool;

pub async fn submit_proposal(
    pool: &PgPool,
    freelancer_id: i32,
    job_id: i32,
    hourly_rate: Option<f64>,
    project_rate: Option<String>,
    description: String,
) -> Result<Proposal, String> {
    let proposal = Proposal {
        id: 0,  // Database will auto-generate the ID
        freelancer_id,
        job_id,
        hourly_rate,
        project_rate,
        description,
        created_at: chrono::Utc::now().to_string(),
        status: ProposalStatus::Submitted,
    };

    // Insert proposal into the database
    sqlx::query!(
        "INSERT INTO proposals (freelancer_id, job_id, hourly_rate, project_rate, description, created_at, status) 
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
        freelancer_id,
        job_id,
        hourly_rate,
        project_rate,
        description,
        proposal.created_at,
        proposal.status as _,
    )
    .fetch_one(pool)
    .await
    .map(|record| Proposal {
        id: record.id,
        ..proposal
    })
    .map_err(|e| e.to_string())
}

pub async fn review_proposal(
    pool: &PgPool,
    proposal_id: i32,
    status: ProposalStatus,
) -> Result<Proposal, String> {
    // Update the proposal's status to Accepted or Rejected
    sqlx::query!(
        "UPDATE proposals SET status = $1 WHERE id = $2 RETURNING id, freelancer_id, job_id, hourly_rate, project_rate, description, created_at, status",
        status as _,
        proposal_id
    )
    .fetch_one(pool)
    .await
    .map(|record| Proposal {
        id: record.id,
        freelancer_id: record.freelancer_id,
        job_id: record.job_id,
        hourly_rate: record.hourly_rate,
        project_rate: record.project_rate,
        description: record.description,
        created_at: record.created_at,
        status: status,
    })
    .map_err(|e| e.to_string())
}

pub async fn get_proposals_for_job(
    pool: &PgPool,
    job_id: i32,
) -> Result<Vec<Proposal>, String> {
    let proposals = sqlx::query_as!(
        Proposal,
        "SELECT id, freelancer_id, job_id, hourly_rate, project_rate, description, created_at, status FROM proposals WHERE job_id = $1",
        job_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(proposals)
}
