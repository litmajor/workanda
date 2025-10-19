
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CryptoWallet {
    pub id: i32,
    pub user_id: i32,
    pub currency: CryptoCurrency,
    pub address: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "crypto_currency")]
pub enum CryptoCurrency {
    BTC,
    ETH,
    USDT,
    USDC,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CryptoTransaction {
    pub id: i32,
    pub from_wallet_id: Option<i32>,
    pub to_wallet_id: i32,
    pub currency: CryptoCurrency,
    pub amount: Decimal,
    pub tx_hash: String,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_status")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoPaymentRequest {
    pub project_id: i32,
    pub amount: Decimal,
    pub currency: CryptoCurrency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub from_currency: String,
    pub to_currency: String,
    pub rate: Decimal,
    pub timestamp: DateTime<Utc>,
}
