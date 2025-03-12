use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u32,
    pub job_id: u32,
    pub freelancer_id: u32,
    pub hourly_rate: Option<f64>,
    pub project_rate: Option<String>,
    pub bid_amount: f64,
    pub message: String,
    pub status: ProposalStatus,  // e.g., "pending", "accepted", "rejected"
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewProposal {
    pub freelancer_id: u32,
    pub bid_amount: f64,
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ProposalStatus {
    Submitted,
    Accepted,
    Rejected,
}