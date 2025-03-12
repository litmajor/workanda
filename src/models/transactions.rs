use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

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
    pub custom_data: Option<String>,
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