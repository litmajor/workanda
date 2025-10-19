use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::services::wallet::wallet_service::WalletError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CurrencyBalance {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub currency_code: String,
    pub currency_type: String,
    pub balance: rust_decimal::Decimal,
    pub locked_balance: rust_decimal::Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceSummary {
    pub total_balances: Vec<CurrencyBalance>,
    pub total_value_usd: f64,
    pub by_currency_type: HashMap<String, f64>,
}

pub struct BalanceService {
    pool: PgPool,
}

impl BalanceService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_wallet_balances(&self, wallet_id: Uuid) -> Result<Vec<CurrencyBalance>, WalletError> {
        let balances = sqlx::query_as!(
            CurrencyBalance,
            r#"
            SELECT id, wallet_id, currency_code, currency_type, balance, locked_balance
            FROM currency_balances
            WHERE wallet_id = $1
            ORDER BY currency_type, currency_code
            "#,
            wallet_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch balances: {}", e)))?;

        Ok(balances)
    }

    pub async fn get_balance_by_currency(
        &self,
        wallet_id: Uuid,
        currency_code: &str,
    ) -> Result<Option<CurrencyBalance>, WalletError> {
        let balance = sqlx::query_as!(
            CurrencyBalance,
            r#"
            SELECT id, wallet_id, currency_code, currency_type, balance, locked_balance
            FROM currency_balances
            WHERE wallet_id = $1 AND currency_code = $2
            "#,
            wallet_id,
            currency_code
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch balance: {}", e)))?;

        Ok(balance)
    }

    pub async fn update_balance(
        &self,
        wallet_id: Uuid,
        currency_code: &str,
        amount: rust_decimal::Decimal,
    ) -> Result<(), WalletError> {
        sqlx::query!(
            r#"
            UPDATE currency_balances
            SET balance = balance + $1, last_updated = CURRENT_TIMESTAMP
            WHERE wallet_id = $2 AND currency_code = $3
            "#,
            amount,
            wallet_id,
            currency_code
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to update balance: {}", e)))?;

        Ok(())
    }

    pub async fn lock_balance(
        &self,
        wallet_id: Uuid,
        currency_code: &str,
        amount: rust_decimal::Decimal,
    ) -> Result<(), WalletError> {
        let result = sqlx::query!(
            r#"
            UPDATE currency_balances
            SET balance = balance - $1, locked_balance = locked_balance + $1
            WHERE wallet_id = $2 AND currency_code = $3 AND balance >= $1
            "#,
            amount,
            wallet_id,
            currency_code
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to lock balance: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(WalletError::ValidationError("Insufficient balance".to_string()));
        }

        Ok(())
    }

    pub async fn unlock_balance(
        &self,
        wallet_id: Uuid,
        currency_code: &str,
        amount: rust_decimal::Decimal,
    ) -> Result<(), WalletError> {
        sqlx::query!(
            r#"
            UPDATE currency_balances
            SET balance = balance + $1, locked_balance = locked_balance - $1
            WHERE wallet_id = $2 AND currency_code = $3 AND locked_balance >= $1
            "#,
            amount,
            wallet_id,
            currency_code
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to unlock balance: {}", e)))?;

        Ok(())
    }

    pub async fn get_total_portfolio_value(&self, wallet_id: Uuid) -> Result<BalanceSummary, WalletError> {
        let balances = self.get_wallet_balances(wallet_id).await?;
        
        let mut by_type: HashMap<String, f64> = HashMap::new();
        let mut total_usd = 0.0;

        for balance in &balances {
            let value = balance.balance.to_string().parse::<f64>().unwrap_or(0.0);
            *by_type.entry(balance.currency_type.clone()).or_insert(0.0) += value;
            total_usd += value;
        }

        Ok(BalanceSummary {
            total_balances: balances,
            total_value_usd: total_usd,
            by_currency_type: by_type,
        })
    }
}
