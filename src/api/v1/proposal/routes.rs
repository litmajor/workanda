use sqlx::PgPool;
use crate::models::proposal::NewProposal;
use crate::models::proposal::Proposal;


pub async fn submit_proposal(
    pool: web::Data<PgPool>,
    web::Path(job_id): web::Path<u32>,
    proposal: web::Json<NewProposal>,
) -> Result<HttpResponse, ApiError> {
    let new_proposal = proposal.into_inner();

    let created_proposal = sqlx::query_as::<_, Proposal>(
        r#"
        INSERT INTO proposals (job_id, freelancer_id, bid_amount, message, status)
        VALUES ($1, $2, $3, $4, 'pending')
        RETURNING *
        "#,
    )
    .bind(job_id)
    .bind(new_proposal.freelancer_id)
    .bind(new_proposal.bid_amount)
    .bind(new_proposal.message)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Proposal submitted successfully for job ID {}: {:?}", job_id, created_proposal);
    Ok(HttpResponse::Created().json(created_proposal))
}

pub async fn get_proposals(
    pool: web::Data<PgPool>,
    web::Path(job_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let proposals = sqlx::query_as::<_, Proposal>("SELECT * FROM proposals WHERE job_id = $1")
        .bind(job_id)
        .fetch_all(pool.as_ref())
        .await?;

    info!("Retrieved proposals for job ID {} successfully", job_id);
    Ok(HttpResponse::Ok().json(proposals))
}

pub async fn select_proposal(
    pool: web::Data<PgPool>,
    web::Path(proposal_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE proposals
        SET status = 'accepted'
        WHERE id = $1 AND status = 'pending'
        "#,
    )
    .bind(proposal_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound("Proposal not found or already accepted".to_string()));
    }

    info!("Proposal selected successfully for proposal ID {}", proposal_id);
    Ok(HttpResponse::Ok().body("Proposal selected successfully"))
}

pub async fn update_proposal(
    pool: web::Data<PgPool>,
    web::Path(proposal_id): web::Path<u32>,
    proposal: web::Json<NewProposal>,
) -> Result<HttpResponse, ApiError> {
    let updated_proposal = proposal.into_inner();

    let proposal = sqlx::query_as::<_, Proposal>(
        r#"
        UPDATE proposals
        SET bid_amount = COALESCE($2, bid_amount),
            message = COALESCE($3, message)
        WHERE id = $1 AND status = 'pending'
        RETURNING *
        "#,
    )
    .bind(proposal_id)
    .bind(updated_proposal.bid_amount)
    .bind(updated_proposal.message)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or(ApiError::NotFound("Proposal not found or cannot be updated".to_string()))?;

    info!("Proposal updated successfully for proposal ID {}", proposal_id);
    Ok(HttpResponse::Ok().json(proposal))
}


pub async fn delete_proposal(
    pool: web::Data<PgPool>,
    web::Path(proposal_id): web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query("DELETE FROM proposals WHERE id = $1 AND status = 'pending'")
        .bind(proposal_id)
        .execute(pool.as_ref())
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound("Proposal not found or cannot be deleted".to_string()));
    }

    info!("Proposal deleted successfully for proposal ID {}", proposal_id);
    Ok(HttpResponse::NoContent())
}