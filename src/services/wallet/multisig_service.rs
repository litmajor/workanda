use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::services::wallet::wallet_service::WalletError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletSigner {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub signer_user_id: Uuid,
    pub signer_address: String,
}

#[derive(Debug, Deserialize)]
pub struct AddSignerRequest {
    pub wallet_id: Uuid,
    pub signer_user_id: Uuid,
    pub signer_address: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateThresholdRequest {
    pub wallet_id: Uuid,
    pub threshold: i32,
}

pub struct MultiSigService {
    pool: PgPool,
}

impl MultiSigService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_multisig_wallet(
        &self,
        user_id: Uuid,
        signers: Vec<String>,
        threshold: i32,
        encryption_password: &str,
    ) -> Result<Uuid, WalletError> {
        if threshold <= 0 || threshold as usize > signers.len() {
            return Err(WalletError::ValidationError(
                "Threshold must be between 1 and number of signers".to_string()
            ));
        }

        if signers.is_empty() {
            return Err(WalletError::ValidationError(
                "At least one signer is required".to_string()
            ));
        }

        let wallet_id = Uuid::new_v4();
        let uuid_bytes = wallet_id.as_bytes();
        let placeholder_address = format!("0x{}{}", hex::encode(&uuid_bytes[..16]), hex::encode(&[0u8; 4]));
        let placeholder_key = "multisig_no_private_key";
        let placeholder_hash = "multisig_wallet_hash";

        let mut tx = self.pool.begin().await
            .map_err(|e| WalletError::DatabaseError(format!("Failed to begin transaction: {}", e)))?;

        sqlx::query!(
            r#"
            INSERT INTO wallets (id, user_id, celo_address, encrypted_private_key, seed_phrase_hash, is_multi_sig, multi_sig_threshold)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            wallet_id,
            user_id,
            placeholder_address,
            placeholder_key,
            placeholder_hash,
            true,
            threshold
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to create multisig wallet: {}", e)))?;

        for (index, signer_address) in signers.iter().enumerate() {
            let signer_id = Uuid::new_v4();
            
            sqlx::query!(
                r#"
                INSERT INTO wallet_signers (id, wallet_id, signer_user_id, signer_address)
                VALUES ($1, $2, $3, $4)
                "#,
                signer_id,
                wallet_id,
                user_id,
                signer_address
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| WalletError::DatabaseError(format!("Failed to insert signer {}: {}", index, e)))?;
        }

        tx.commit().await
            .map_err(|e| WalletError::DatabaseError(format!("Failed to commit transaction: {}", e)))?;

        Ok(wallet_id)
    }

    pub async fn add_signer(&self, request: AddSignerRequest) -> Result<WalletSigner, WalletError> {
        let wallet = sqlx::query!(
            "SELECT is_multi_sig FROM wallets WHERE id = $1",
            request.wallet_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch wallet: {}", e)))?;

        match wallet {
            None => return Err(WalletError::ValidationError("Wallet not found".to_string())),
            Some(w) if !w.is_multi_sig => {
                return Err(WalletError::ValidationError("Wallet is not multi-sig".to_string()))
            }
            _ => {}
        }

        let signer_id = Uuid::new_v4();
        
        let signer = sqlx::query_as!(
            WalletSigner,
            r#"
            INSERT INTO wallet_signers (id, wallet_id, signer_user_id, signer_address)
            VALUES ($1, $2, $3, $4)
            RETURNING id, wallet_id, signer_user_id, signer_address
            "#,
            signer_id,
            request.wallet_id,
            request.signer_user_id,
            request.signer_address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to add signer: {}", e)))?;

        Ok(signer)
    }

    pub async fn remove_signer(
        &self,
        wallet_id: Uuid,
        signer_id: Uuid,
    ) -> Result<(), WalletError> {
        let result = sqlx::query!(
            "DELETE FROM wallet_signers WHERE wallet_id = $1 AND id = $2",
            wallet_id,
            signer_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to remove signer: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(WalletError::ValidationError("Signer not found".to_string()));
        }

        let signer_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM wallet_signers WHERE wallet_id = $1",
            wallet_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to count signers: {}", e)))?;

        let threshold = sqlx::query_scalar!(
            "SELECT multi_sig_threshold FROM wallets WHERE id = $1",
            wallet_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch threshold: {}", e)))?;

        if let (Some(count), Some(thresh)) = (signer_count, threshold) {
            if (count as i32) < thresh {
                return Err(WalletError::ValidationError(
                    "Cannot remove signer: would fall below threshold".to_string()
                ));
            }
        }

        Ok(())
    }

    pub async fn update_threshold(&self, request: UpdateThresholdRequest) -> Result<(), WalletError> {
        let signer_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM wallet_signers WHERE wallet_id = $1",
            request.wallet_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to count signers: {}", e)))?;

        if let Some(count) = signer_count {
            if request.threshold <= 0 || (request.threshold as i64) > count {
                return Err(WalletError::ValidationError(
                    "Threshold must be between 1 and number of signers".to_string()
                ));
            }
        }

        sqlx::query!(
            "UPDATE wallets SET multi_sig_threshold = $1 WHERE id = $2",
            request.threshold,
            request.wallet_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to update threshold: {}", e)))?;

        Ok(())
    }

    pub async fn get_signers(&self, wallet_id: Uuid) -> Result<Vec<WalletSigner>, WalletError> {
        let signers = sqlx::query_as!(
            WalletSigner,
            r#"
            SELECT id, wallet_id, signer_user_id, signer_address
            FROM wallet_signers
            WHERE wallet_id = $1
            "#,
            wallet_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch signers: {}", e)))?;

        Ok(signers)
    }
}
