
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TeamProposal {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: u32,
    pub bid_amount: f64,
    pub message: String,
    pub proposed_revenue_distribution: serde_json::Value,
    pub status: TeamProposalStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTeamProposal {
    pub team_id: Uuid,
    pub job_id: u32,
    pub bid_amount: f64,
    pub message: String,
    pub revenue_distribution: Vec<RevenueShare>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TeamProposalStatus {
    Pending,
    Accepted,
    Rejected,
    Withdrawn,
}

impl ToString for TeamProposalStatus {
    fn to_string(&self) -> String {
        match self {
            TeamProposalStatus::Pending => "pending".to_string(),
            TeamProposalStatus::Accepted => "accepted".to_string(),
            TeamProposalStatus::Rejected => "rejected".to_string(),
            TeamProposalStatus::Withdrawn => "withdrawn".to_string(),
        }
    }
}
