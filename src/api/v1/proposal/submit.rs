// api/proposal/submit.rs
use crate::services::proposal_service::submit_proposal;
use crate::models::ProposalStatus;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubmitProposalRequest {
    pub freelancer_id: i32,
    pub job_id: i32,
    pub hourly_rate: Option<f64>,
    pub project_rate: Option<String>,
    pub description: String,
}

pub async fn submit_proposal_handler(
    pool: web::Data<sqlx::PgPool>,
    body: web::Json<SubmitProposalRequest>,
) -> impl Responder {
    let result = submit_proposal(
        pool.get_ref(),
        body.freelancer_id,
        body.job_id,
        body.hourly_rate,
        body.project_rate.clone(),
        body.description.clone(),
    )
    .await;

    match result {
        Ok(proposal) => HttpResponse::Ok().json(proposal),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
