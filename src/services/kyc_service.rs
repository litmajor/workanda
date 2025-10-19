
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use crate::models::kyc::{KycVerification, KycLevel, KycStatus, SubmitKycRequest, KycLimits};
use crate::api::error::ApiError;

pub struct KycService {
    pool: PgPool,
}

impl KycService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Submit KYC verification request
    pub async fn submit_kyc(
        &self,
        user_id: i32,
        request: SubmitKycRequest,
    ) -> Result<KycVerification, ApiError> {
        // Validate documents based on level
        self.validate_documents(&request)?;

        let id = Uuid::new_v4();
        let expires_at = match request.verification_level {
            KycLevel::Basic => Some(Utc::now() + Duration::days(365)),
            KycLevel::Intermediate => Some(Utc::now() + Duration::days(730)),
            KycLevel::Advanced => Some(Utc::now() + Duration::days(1095)),
            KycLevel::None => None,
        };

        let kyc = sqlx::query_as!(
            KycVerification,
            r#"
            INSERT INTO kyc_verifications (
                id, user_id, verification_level, status, 
                id_document_url, proof_of_address_url, selfie_url,
                expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING 
                id, user_id, 
                verification_level as "verification_level: KycLevel", 
                status as "status: KycStatus",
                id_document_url, proof_of_address_url, selfie_url,
                verification_provider, provider_verification_id,
                verified_at, expires_at, rejection_reason,
                created_at, updated_at
            "#,
            id,
            user_id,
            request.verification_level as KycLevel,
            KycStatus::Pending as KycStatus,
            request.id_document_url,
            request.proof_of_address_url,
            request.selfie_url,
            expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Failed to submit KYC: {}", e)))?;

        Ok(kyc)
    }

    /// Get user's current KYC status
    pub async fn get_kyc_status(
        &self,
        user_id: i32,
    ) -> Result<Option<KycVerification>, ApiError> {
        let kyc = sqlx::query_as!(
            KycVerification,
            r#"
            SELECT 
                id, user_id, 
                verification_level as "verification_level: KycLevel", 
                status as "status: KycStatus",
                id_document_url, proof_of_address_url, selfie_url,
                verification_provider, provider_verification_id,
                verified_at, expires_at, rejection_reason,
                created_at, updated_at
            FROM kyc_verifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Failed to fetch KYC: {}", e)))?;

        Ok(kyc)
    }

    /// Approve KYC verification (admin only)
    pub async fn approve_kyc(
        &self,
        kyc_id: Uuid,
        provider: Option<String>,
        provider_verification_id: Option<String>,
    ) -> Result<KycVerification, ApiError> {
        let kyc = sqlx::query_as!(
            KycVerification,
            r#"
            UPDATE kyc_verifications
            SET 
                status = $1,
                verified_at = $2,
                verification_provider = $3,
                provider_verification_id = $4,
                updated_at = NOW()
            WHERE id = $5
            RETURNING 
                id, user_id, 
                verification_level as "verification_level: KycLevel", 
                status as "status: KycStatus",
                id_document_url, proof_of_address_url, selfie_url,
                verification_provider, provider_verification_id,
                verified_at, expires_at, rejection_reason,
                created_at, updated_at
            "#,
            KycStatus::Approved as KycStatus,
            Some(Utc::now()),
            provider,
            provider_verification_id,
            kyc_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Failed to approve KYC: {}", e)))?;

        Ok(kyc)
    }

    /// Reject KYC verification (admin only)
    pub async fn reject_kyc(
        &self,
        kyc_id: Uuid,
        reason: String,
    ) -> Result<KycVerification, ApiError> {
        let kyc = sqlx::query_as!(
            KycVerification,
            r#"
            UPDATE kyc_verifications
            SET 
                status = $1,
                rejection_reason = $2,
                updated_at = NOW()
            WHERE id = $3
            RETURNING 
                id, user_id, 
                verification_level as "verification_level: KycLevel", 
                status as "status: KycStatus",
                id_document_url, proof_of_address_url, selfie_url,
                verification_provider, provider_verification_id,
                verified_at, expires_at, rejection_reason,
                created_at, updated_at
            "#,
            KycStatus::Rejected as KycStatus,
            Some(reason),
            kyc_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Failed to reject KYC: {}", e)))?;

        Ok(kyc)
    }

    /// Get transaction limits based on KYC level
    pub async fn get_limits(&self, user_id: i32) -> Result<KycLimits, ApiError> {
        let kyc = self.get_kyc_status(user_id).await?;
        
        let level = match kyc {
            Some(verification) if verification.status == KycStatus::Approved => {
                verification.verification_level
            }
            _ => KycLevel::None,
        };

        Ok(level.get_limits())
    }

    /// Check if user can perform transaction based on KYC limits
    pub async fn can_transact(
        &self,
        user_id: i32,
        amount: rust_decimal::Decimal,
    ) -> Result<bool, ApiError> {
        let limits = self.get_limits(user_id).await?;
        Ok(amount <= limits.daily_deposit_limit)
    }

    /// Check if KYC is expired
    pub async fn check_expiration(&self) -> Result<(), ApiError> {
        sqlx::query!(
            r#"
            UPDATE kyc_verifications
            SET status = $1
            WHERE expires_at < NOW() 
            AND status = $2
            "#,
            KycStatus::Expired as KycStatus,
            KycStatus::Approved as KycStatus
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Failed to check expiration: {}", e)))?;

        Ok(())
    }

    /// Get all pending KYC verifications (admin)
    pub async fn get_pending_verifications(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<KycVerification>, ApiError> {
        let verifications = sqlx::query_as!(
            KycVerification,
            r#"
            SELECT 
                id, user_id, 
                verification_level as "verification_level: KycLevel", 
                status as "status: KycStatus",
                id_document_url, proof_of_address_url, selfie_url,
                verification_provider, provider_verification_id,
                verified_at, expires_at, rejection_reason,
                created_at, updated_at
            FROM kyc_verifications
            WHERE status = $1
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
            KycStatus::Pending as KycStatus,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Failed to fetch pending KYC: {}", e)))?;

        Ok(verifications)
    }

    /// Validate documents based on KYC level
    fn validate_documents(&self, request: &SubmitKycRequest) -> Result<(), ApiError> {
        match request.verification_level {
            KycLevel::Basic => {
                // Basic only needs email/phone (handled elsewhere)
                Ok(())
            }
            KycLevel::Intermediate => {
                if request.id_document_url.is_none() || request.selfie_url.is_none() {
                    return Err(ApiError::BadRequest(
                        "ID document and selfie required for Intermediate KYC".to_string()
                    ));
                }
                Ok(())
            }
            KycLevel::Advanced => {
                if request.id_document_url.is_none() 
                    || request.selfie_url.is_none() 
                    || request.proof_of_address_url.is_none() {
                    return Err(ApiError::BadRequest(
                        "ID document, selfie, and proof of address required for Advanced KYC".to_string()
                    ));
                }
                Ok(())
            }
            KycLevel::None => Ok(()),
        }
    }

    /// Get KYC statistics (admin)
    pub async fn get_statistics(&self) -> Result<KycStatistics, ApiError> {
        let stats = sqlx::query_as!(
            KycStatistics,
            r#"
            SELECT 
                COUNT(*) FILTER (WHERE status = 'Pending') as "pending!",
                COUNT(*) FILTER (WHERE status = 'Approved') as "approved!",
                COUNT(*) FILTER (WHERE status = 'Rejected') as "rejected!",
                COUNT(*) FILTER (WHERE status = 'Expired') as "expired!",
                COUNT(*) FILTER (WHERE verification_level = 'Basic') as "basic!",
                COUNT(*) FILTER (WHERE verification_level = 'Intermediate') as "intermediate!",
                COUNT(*) FILTER (WHERE verification_level = 'Advanced') as "advanced!"
            FROM kyc_verifications
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(format!("Failed to fetch KYC stats: {}", e)))?;

        Ok(stats)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct KycStatistics {
    pub pending: i64,
    pub approved: i64,
    pub rejected: i64,
    pub expired: i64,
    pub basic: i64,
    pub intermediate: i64,
    pub advanced: i64,
}
