
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CurrencySwap {
    pub id: Uuid,
    pub wallet_id: i32,
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount: Decimal,
    pub to_amount: Decimal,
    pub exchange_rate: Decimal,
    pub slippage_tolerance: Decimal,
    pub actual_slippage: Option<Decimal>,
    pub swap_provider: SwapProvider,
    pub tx_hash: Option<String>,
    pub fees: Decimal,
    pub status: SwapStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "swap_provider")]
pub enum SwapProvider {
    Ubeswap,
    Curve,
    Internal, // For stable pairs like cUSD <-> USDT
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "swap_status")]
pub enum SwapStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSwapRequest {
    pub wallet_id: i32,
    pub from_currency: String,
    pub to_currency: String,
    pub amount: Decimal,
    pub slippage_tolerance: Option<Decimal>, // Default 0.5%
    pub auto_approve: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapQuote {
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount: Decimal,
    pub estimated_to_amount: Decimal,
    pub exchange_rate: Decimal,
    pub price_impact: Decimal,
    pub fees: Decimal,
    pub route: Vec<String>,
    pub provider: SwapProvider,
    pub valid_until: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AutoConversionPreference {
    pub id: Uuid,
    pub wallet_id: i32,
    pub enabled: bool,
    pub target_currency: String,
    pub minimum_amount: Decimal,
    pub convert_on_receive: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapAnalytics {
    pub total_swaps: i64,
    pub total_volume_usd: Decimal,
    pub average_slippage: Decimal,
    pub most_swapped_pairs: Vec<(String, String, i64)>,
    pub total_fees_paid: Decimal,
}
