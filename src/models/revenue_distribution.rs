
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevenueShare {
    pub freelancer_id: Uuid,
    pub percentage: f64,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RevenueDistribution {
    pub id: Uuid,
    pub team_id: Uuid,
    pub contract_id: i32,
    pub total_amount: f64,
    pub distribution_plan: serde_json::Value,
    pub status: DistributionStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRevenueDistribution {
    pub team_id: Uuid,
    pub contract_id: i32,
    pub total_amount: f64,
    pub distribution_plan: Vec<RevenueShare>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DistributionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl ToString for DistributionStatus {
    fn to_string(&self) -> String {
        match self {
            DistributionStatus::Pending => "pending".to_string(),
            DistributionStatus::Processing => "processing".to_string(),
            DistributionStatus::Completed => "completed".to_string(),
            DistributionStatus::Failed => "failed".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DistributionPayment {
    pub id: Uuid,
    pub distribution_id: Uuid,
    pub freelancer_id: Uuid,
    pub amount: f64,
    pub status: String,
    pub transaction_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
