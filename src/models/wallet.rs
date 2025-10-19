
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
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub celo_address: String,
    pub encrypted_private_key: String,
    pub mnemonic_encrypted: Option<String>,
    pub wallet_type: WalletType,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "wallet_type")]
pub enum WalletType {
    Individual,
    Team,
    Escrow,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WalletBalance {
    pub id: i32,
    pub wallet_id: i32,
    pub currency_code: String,
    pub balance: Decimal,
    pub locked_balance: Decimal,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WalletTransaction {
    pub id: i32,
    pub from_wallet_id: Option<i32>,
    pub to_wallet_id: Option<i32>,
    pub to_address: Option<String>,
    pub amount: Decimal,
    pub currency_code: String,
    pub tx_hash: Option<String>,
    pub status: TransactionStatus,
    pub tx_type: TransactionType,
    pub gas_fee: Option<Decimal>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_status")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_type")]
pub enum TransactionType {
    Payment,
    EscrowDeposit,
    MilestoneRelease,
    Swap,
    Deposit,
    Withdrawal,
    Staking,
    Unstaking,
    Reward,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SavingsAccount {
    pub id: i32,
    pub wallet_id: i32,
    pub account_type: SavingsType,
    pub target_amount: Option<Decimal>,
    pub current_balance: Decimal,
    pub currency_code: String,
    pub interest_rate: Decimal,
    pub unlock_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "savings_type")]
pub enum SavingsType {
    Flexible,
    Locked,
    GoalBased,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StakingPosition {
    pub id: i32,
    pub wallet_id: i32,
    pub protocol: String,
    pub staked_amount: Decimal,
    pub currency_code: String,
    pub rewards_earned: Decimal,
    pub apy: Decimal,
    pub staked_at: DateTime<Utc>,
    pub unstaked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub user_id: i32,
    pub wallet_type: WalletType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMoneyRequest {
    pub from_wallet_id: i32,
    pub to_address: String, // can be celo address or username
    pub amount: Decimal,
    pub currency_code: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapRequest {
    pub wallet_id: i32,
    pub from_currency: String,
    pub to_currency: String,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletStats {
    pub total_balance_usd: Decimal,
    pub balances: Vec<WalletBalance>,
    pub pending_transactions: i32,
    pub monthly_income: Decimal,
    pub monthly_expenses: Decimal,
}
