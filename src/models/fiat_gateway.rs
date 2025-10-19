
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FiatTransaction {
    pub id: Uuid,
    pub wallet_id: i32,
    pub transaction_type: FiatTransactionType,
    pub amount: Decimal,
    pub currency_code: String,
    pub payment_method: PaymentMethod,
    pub payment_provider: PaymentProvider,
    pub provider_transaction_id: Option<String>,
    pub status: FiatTransactionStatus,
    pub fees: Decimal,
    pub destination_address: Option<String>, // Bank account or mobile money number
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "fiat_transaction_type")]
pub enum FiatTransactionType {
    Deposit,    // Buy crypto with fiat
    Withdrawal, // Sell crypto for fiat
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "payment_method")]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    BankTransfer,
    MobileMoney,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "payment_provider")]
pub enum PaymentProvider {
    Stripe,
    Flutterwave,
    Paystack,
    MPesa,
    MTNMobileMoney,
    AirtelMoney,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "fiat_transaction_status")]
pub enum FiatTransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositFiatRequest {
    pub wallet_id: i32,
    pub amount: Decimal,
    pub currency_code: String,
    pub payment_method: PaymentMethod,
    pub payment_provider: PaymentProvider,
    pub card_details: Option<CardDetails>,
    pub mobile_money_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawFiatRequest {
    pub wallet_id: i32,
    pub amount: Decimal,
    pub currency_code: String,
    pub payment_method: PaymentMethod,
    pub payment_provider: PaymentProvider,
    pub bank_account: Option<BankAccount>,
    pub mobile_money_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardDetails {
    pub card_number: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvv: String,
    pub cardholder_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    pub account_number: String,
    pub account_name: String,
    pub bank_name: String,
    pub bank_code: Option<String>,
    pub swift_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProviderResponse {
    pub success: bool,
    pub transaction_id: String,
    pub payment_url: Option<String>,
    pub message: Option<String>,
}
