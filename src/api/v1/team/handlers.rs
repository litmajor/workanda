
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::team::Team;
use crate::models::team_proposal::{NewTeamProposal, TeamProposal};
use crate::models::revenue_distribution::{NewRevenueDistribution, RevenueDistribution};
use crate::api::error::ApiError;

// Create a new team
pub async fn create_team_handler(
    pool: web::Data<PgPool>,
    team: web::Json<Team>,
) -> Result<HttpResponse, ApiError> {
    let team_id = Uuid::new_v4();
    
    let created_team = sqlx::query_as::<_, Team>(
        r#"
        INSERT INTO teams (id, name, leader_id, skills, available, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        RETURNING *
        "#
    )
    .bind(team_id)
    .bind(&team.name)
    .bind(team.leader_id)
    .bind(&team.skills)
    .bind(team.available)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(created_team))
}

// Get team by ID
pub async fn get_team_handler(
    pool: web::Data<PgPool>,
    team_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let team = sqlx::query_as::<_, Team>("SELECT * FROM teams WHERE id = $1")
        .bind(*team_id)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .ok_or(ApiError::NotFound(format!("Team {} not found", team_id)))?;

    Ok(HttpResponse::Ok().json(team))
}

// Update team
pub async fn update_team_handler(
    pool: web::Data<PgPool>,
    team_id: web::Path<Uuid>,
    team: web::Json<Team>,
) -> Result<HttpResponse, ApiError> {
    let updated_team = sqlx::query_as::<_, Team>(
        r#"
        UPDATE teams
        SET name = $2, skills = $3, available = $4
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(*team_id)
    .bind(&team.name)
    .bind(&team.skills)
    .bind(team.available)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(updated_team))
}

// Submit team proposal
pub async fn submit_team_proposal_handler(
    pool: web::Data<PgPool>,
    proposal: web::Json<NewTeamProposal>,
) -> Result<HttpResponse, ApiError> {
    let proposal_id = Uuid::new_v4();
    let revenue_json = serde_json::to_value(&proposal.revenue_distribution)
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let created_proposal = sqlx::query_as::<_, TeamProposal>(
        r#"
        INSERT INTO team_proposals (id, team_id, job_id, bid_amount, message, proposed_revenue_distribution, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending', NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(proposal_id)
    .bind(proposal.team_id)
    .bind(proposal.job_id)
    .bind(proposal.bid_amount)
    .bind(&proposal.message)
    .bind(revenue_json)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(created_proposal))
}

// Get team proposals for a job
pub async fn get_team_proposals_handler(
    pool: web::Data<PgPool>,
    job_id: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let proposals = sqlx::query_as::<_, TeamProposal>(
        "SELECT * FROM team_proposals WHERE job_id = $1"
    )
    .bind(*job_id)
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(proposals))
}

// Create revenue distribution
pub async fn create_revenue_distribution_handler(
    pool: web::Data<PgPool>,
    distribution: web::Json<NewRevenueDistribution>,
) -> Result<HttpResponse, ApiError> {
    let distribution_id = Uuid::new_v4();
    let plan_json = serde_json::to_value(&distribution.distribution_plan)
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let created_distribution = sqlx::query_as::<_, RevenueDistribution>(
        r#"
        INSERT INTO revenue_distributions (id, team_id, contract_id, total_amount, distribution_plan, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 'pending', NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(distribution_id)
    .bind(distribution.team_id)
    .bind(distribution.contract_id)
    .bind(distribution.total_amount)
    .bind(plan_json)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(created_distribution))
}

// Process revenue distribution
pub async fn process_revenue_distribution_handler(
    pool: web::Data<PgPool>,
    distribution_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let distribution = sqlx::query_as::<_, RevenueDistribution>(
        "SELECT * FROM revenue_distributions WHERE id = $1"
    )
    .bind(*distribution_id)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?
    .ok_or(ApiError::NotFound(format!("Distribution {} not found", distribution_id)))?;

    // Update status to processing
    sqlx::query(
        "UPDATE revenue_distributions SET status = 'processing', updated_at = NOW() WHERE id = $1"
    )
    .bind(*distribution_id)
    .execute(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Revenue distribution processing started",
        "distribution_id": distribution_id
    })))
}
