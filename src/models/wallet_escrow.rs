
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WalletEscrow {
    pub id: i32,
    pub wallet_id: i32,
    pub project_id: i32,
    pub contract_id: i32,
    pub amount: Decimal,
    pub currency_code: String,
    pub status: EscrowStatus,
    pub locked_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "wallet_escrow_status")]
pub enum EscrowStatus {
    Locked,
    Released,
    Refunded,
    Disputed,
    Frozen,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EscrowMilestone {
    pub id: i32,
    pub escrow_id: i32,
    pub milestone_id: i32,
    pub amount: Decimal,
    pub status: MilestoneEscrowStatus,
    pub released_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "milestone_escrow_status")]
pub enum MilestoneEscrowStatus {
    Pending,
    Approved,
    Released,
    Disputed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEscrowRequest {
    pub wallet_id: i32,
    pub project_id: i32,
    pub contract_id: i32,
    pub amount: Decimal,
    pub currency_code: String,
    pub milestones: Vec<MilestoneAmount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MilestoneAmount {
    pub milestone_id: i32,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseMilestoneRequest {
    pub escrow_id: i32,
    pub milestone_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisputeEscrowRequest {
    pub escrow_id: i32,
    pub reason: String,
    pub freeze_wallet: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundEscrowRequest {
    pub escrow_id: i32,
    pub reason: String,
    pub partial_amount: Option<Decimal>,
}
