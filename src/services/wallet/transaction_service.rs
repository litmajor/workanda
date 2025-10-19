use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::services::wallet::wallet_service::WalletError;
use crate::services::wallet::balance_service::BalanceService;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WalletTransaction {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_hash: Option<String>,
    pub transaction_type: String,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub currency_code: String,
    pub amount: rust_decimal::Decimal,
    pub gas_fee: Option<rust_decimal::Decimal>,
    pub status: String,
    pub block_number: Option<i64>,
    pub confirmations: i32,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub wallet_id: Uuid,
    pub transaction_type: String,
    pub to_address: String,
    pub currency_code: String,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize)]
pub struct TransactionFilter {
    pub wallet_id: Uuid,
    pub status: Option<String>,
    pub transaction_type: Option<String>,
    pub currency_code: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
}

pub struct TransactionService {
    pool: PgPool,
}

impl TransactionService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_transaction(
        &self,
        request: CreateTransactionRequest,
        from_address: &str,
    ) -> Result<WalletTransaction, WalletError> {
        let balance_service = BalanceService::new(self.pool.clone());
        
        balance_service
            .lock_balance(request.wallet_id, &request.currency_code, request.amount)
            .await?;

        let transaction_id = Uuid::new_v4();
        
        let transaction = sqlx::query_as!(
            WalletTransaction,
            r#"
            INSERT INTO wallet_transactions 
            (id, wallet_id, transaction_type, from_address, to_address, currency_code, amount, status, confirmations)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', 0)
            RETURNING id, wallet_id, transaction_hash, transaction_type, from_address, to_address, 
                      currency_code, amount, gas_fee, status, block_number, confirmations, 
                      created_at, confirmed_at
            "#,
            transaction_id,
            request.wallet_id,
            request.transaction_type,
            from_address,
            request.to_address,
            request.currency_code,
            request.amount
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to create transaction: {}", e)))?;

        Ok(transaction)
    }

    pub async fn update_transaction_status(
        &self,
        transaction_id: Uuid,
        status: &str,
        transaction_hash: Option<String>,
        block_number: Option<i64>,
    ) -> Result<(), WalletError> {
        sqlx::query!(
            r#"
            UPDATE wallet_transactions
            SET status = $1, transaction_hash = $2, block_number = $3, 
                confirmed_at = CASE WHEN $1 = 'confirmed' THEN CURRENT_TIMESTAMP ELSE confirmed_at END
            WHERE id = $4
            "#,
            status,
            transaction_hash,
            block_number,
            transaction_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to update transaction: {}", e)))?;

        if status == "confirmed" {
            let transaction = self.get_transaction_by_id(transaction_id).await?;
            if let Some(tx) = transaction {
                let balance_service = BalanceService::new(self.pool.clone());
                
                balance_service
                    .unlock_balance(tx.wallet_id, &tx.currency_code, tx.amount)
                    .await?;
                
                if tx.transaction_type == "send" {
                    balance_service
                        .update_balance(tx.wallet_id, &tx.currency_code, -tx.amount)
                        .await?;
                }
            }
        } else if status == "failed" || status == "cancelled" {
            let transaction = self.get_transaction_by_id(transaction_id).await?;
            if let Some(tx) = transaction {
                let balance_service = BalanceService::new(self.pool.clone());
                balance_service
                    .unlock_balance(tx.wallet_id, &tx.currency_code, tx.amount)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn receive_payment(
        &self,
        wallet_id: Uuid,
        from_address: &str,
        currency_code: &str,
        amount: rust_decimal::Decimal,
        transaction_hash: &str,
    ) -> Result<WalletTransaction, WalletError> {
        let transaction_id = Uuid::new_v4();
        
        let transaction = sqlx::query_as!(
            WalletTransaction,
            r#"
            INSERT INTO wallet_transactions 
            (id, wallet_id, transaction_type, from_address, currency_code, amount, transaction_hash, status, confirmations)
            VALUES ($1, $2, 'receive', $3, $4, $5, $6, 'confirmed', 1)
            RETURNING id, wallet_id, transaction_hash, transaction_type, from_address, to_address, 
                      currency_code, amount, gas_fee, status, block_number, confirmations, 
                      created_at, confirmed_at
            "#,
            transaction_id,
            wallet_id,
            from_address,
            currency_code,
            amount,
            transaction_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to record received payment: {}", e)))?;

        let balance_service = BalanceService::new(self.pool.clone());
        balance_service
            .update_balance(wallet_id, currency_code, amount)
            .await?;

        Ok(transaction)
    }

    pub async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<WalletTransaction>, WalletError> {
        let transaction = sqlx::query_as!(
            WalletTransaction,
            r#"
            SELECT id, wallet_id, transaction_hash, transaction_type, from_address, to_address, 
                   currency_code, amount, gas_fee, status, block_number, confirmations, 
                   created_at, confirmed_at
            FROM wallet_transactions
            WHERE id = $1
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch transaction: {}", e)))?;

        Ok(transaction)
    }

    pub async fn get_transactions(&self, filter: TransactionFilter) -> Result<Vec<WalletTransaction>, WalletError> {
        let limit = filter.limit.unwrap_or(50);
        
        let mut query = String::from(
            "SELECT id, wallet_id, transaction_hash, transaction_type, from_address, to_address, 
             currency_code, amount, gas_fee, status, block_number, confirmations, created_at, confirmed_at 
             FROM wallet_transactions WHERE wallet_id = $1"
        );
        
        let mut param_count = 1;
        
        if filter.status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
        }
        
        if filter.transaction_type.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND transaction_type = ${}", param_count));
        }
        
        if filter.currency_code.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND currency_code = ${}", param_count));
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT $");
        param_count += 1;
        query.push_str(&param_count.to_string());
        
        let mut query_builder = sqlx::query_as::<_, WalletTransaction>(&query);
        query_builder = query_builder.bind(filter.wallet_id);
        
        if let Some(status) = filter.status {
            query_builder = query_builder.bind(status);
        }
        if let Some(transaction_type) = filter.transaction_type {
            query_builder = query_builder.bind(transaction_type);
        }
        if let Some(currency_code) = filter.currency_code {
            query_builder = query_builder.bind(currency_code);
        }
        query_builder = query_builder.bind(limit);
        
        let transactions = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch transactions: {}", e)))?;

        Ok(transactions)
    }

    pub async fn get_transaction_history(
        &self,
        wallet_id: Uuid,
        limit: i64,
    ) -> Result<Vec<WalletTransaction>, WalletError> {
        let transactions = sqlx::query_as!(
            WalletTransaction,
            r#"
            SELECT id, wallet_id, transaction_hash, transaction_type, from_address, to_address, 
                   currency_code, amount, gas_fee, status, block_number, confirmations, 
                   created_at, confirmed_at
            FROM wallet_transactions
            WHERE wallet_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            wallet_id,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch transaction history: {}", e)))?;

        Ok(transactions)
    }
}
