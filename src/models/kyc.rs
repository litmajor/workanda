
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct KycVerification {
    pub id: Uuid,
    pub user_id: i32,
    pub verification_level: KycLevel,
    pub status: KycStatus,
    pub id_document_url: Option<String>,
    pub proof_of_address_url: Option<String>,
    pub selfie_url: Option<String>,
    pub verification_provider: Option<String>,
    pub provider_verification_id: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "kyc_level")]
pub enum KycLevel {
    None,       // No KYC, limited to $100/day
    Basic,      // Email + phone verified, $1,000/day
    Intermediate, // ID document verified, $10,000/day
    Advanced,   // Full KYC with proof of address, $100,000/day
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "kyc_status")]
pub enum KycStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
    Expired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KycLimits {
    pub daily_deposit_limit: rust_decimal::Decimal,
    pub daily_withdrawal_limit: rust_decimal::Decimal,
    pub monthly_limit: rust_decimal::Decimal,
}

impl KycLevel {
    pub fn get_limits(&self) -> KycLimits {
        use rust_decimal::Decimal;
        use std::str::FromStr;
        
        match self {
            KycLevel::None => KycLimits {
                daily_deposit_limit: Decimal::from_str("100").unwrap(),
                daily_withdrawal_limit: Decimal::from_str("100").unwrap(),
                monthly_limit: Decimal::from_str("1000").unwrap(),
            },
            KycLevel::Basic => KycLimits {
                daily_deposit_limit: Decimal::from_str("1000").unwrap(),
                daily_withdrawal_limit: Decimal::from_str("1000").unwrap(),
                monthly_limit: Decimal::from_str("10000").unwrap(),
            },
            KycLevel::Intermediate => KycLimits {
                daily_deposit_limit: Decimal::from_str("10000").unwrap(),
                daily_withdrawal_limit: Decimal::from_str("10000").unwrap(),
                monthly_limit: Decimal::from_str("100000").unwrap(),
            },
            KycLevel::Advanced => KycLimits {
                daily_deposit_limit: Decimal::from_str("100000").unwrap(),
                daily_withdrawal_limit: Decimal::from_str("100000").unwrap(),
                monthly_limit: Decimal::from_str("1000000").unwrap(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitKycRequest {
    pub verification_level: KycLevel,
    pub id_document_url: Option<String>,
    pub proof_of_address_url: Option<String>,
    pub selfie_url: Option<String>,
}
