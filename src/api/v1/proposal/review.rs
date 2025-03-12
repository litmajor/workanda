// api/proposal/review.rs
use crate::services::proposal_service::review_proposal;
use crate::models::ProposalStatus;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReviewProposalRequest {
    pub proposal_id: i32,
    pub status: ProposalStatus,  // Can be "Accepted" or "Rejected"
}

pub async fn review_proposal_handler(
    pool: web::Data<sqlx::PgPool>,
    body: web::Json<ReviewProposalRequest>,
) -> impl Responder {
    let result = review_proposal(pool.get_ref(), body.proposal_id, body.status).await;

    match result {
        Ok(proposal) => HttpResponse::Ok().json(proposal),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
