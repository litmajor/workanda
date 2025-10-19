
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub currency: String,
    pub balance: Decimal,
    pub available_balance: Decimal,
    pub locked_balance: Decimal,
    pub wallet_address: Option<String>,
    pub is_primary: bool,
    pub status: WalletStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "wallet_status")]
pub enum WalletStatus {
    Active,
    Frozen,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WalletTransaction {
    pub id: i32,
    pub wallet_id: i32,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub reference_id: Option<String>,
    pub reference_type: Option<String>,
    pub balance_before: Decimal,
    pub balance_after: Decimal,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_type")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    EscrowLock,
    EscrowRelease,
    EscrowRefund,
    Transfer,
    Fee,
    Refund,
    Reward,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_status")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub currency: String,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositRequest {
    pub wallet_id: i32,
    pub amount: Decimal,
    pub payment_method: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    pub wallet_id: i32,
    pub amount: Decimal,
    pub destination: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from_wallet_id: i32,
    pub to_user_id: i32,
    pub amount: Decimal,
    pub currency: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalance {
    pub currency: String,
    pub total_balance: Decimal,
    pub available_balance: Decimal,
    pub locked_balance: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletOverview {
    pub total_balance_usd: Decimal,
    pub wallets: Vec<WalletBalance>,
    pub recent_transactions: Vec<WalletTransaction>,
    pub pending_escrows: Decimal,
}
