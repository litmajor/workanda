use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct EscrowAccount {
    pub id: u32,
    pub contract_id: u32,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub release_conditions: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub released_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct NewEscrowAccount {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub amount: f64,
    pub currency: String,
    pub release_conditions: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub escrow_account_id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub amount: f64,
    pub status: String,
    pub fee: f64,
    pub created_at: NaiveDateTime,
    pub custom_data: Option<String>, // Optional JSON field for custom data
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Deposit,
    Fee,
    Payout,
}

#[derive(Serialize, Deserialize)]
pub struct NewTransaction {
    pub escrow_account_id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub amount: f64,
    pub status: String,
    pub fee: f64,
    pub custom_data: Option<String>,
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentReminder {
    pub id: u32,
    pub contract_id: u32,
    pub milestone_id: u32,
    pub due_date: NaiveDateTime,
    pub sent: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DisputeResolution {
    Refund,
    Release,
    Other, // Example: for cases like mediations
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DisputeLevel {
    InitialReview,
    Mediation,
    Arbitration,
    Resolved,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dispute {
    pub id: u32,
    pub escrow_id: u32,
    pub level: DisputeLevel,
    pub resolution: Option<DisputeResolution>,
}
