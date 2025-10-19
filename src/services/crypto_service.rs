
use sqlx::PgPool;
use rust_decimal::Decimal;
use crate::models::cryptocurrency::*;
use crate::api::error::ApiError;

pub struct CryptoService {
    pool: PgPool,
}

impl CryptoService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_wallet(
        &self,
        user_id: i32,
        currency: CryptoCurrency,
    ) -> Result<CryptoWallet, ApiError> {
        let address = self.generate_wallet_address(&currency)?;
        
        let wallet = sqlx::query_as!(
            CryptoWallet,
            r#"
            INSERT INTO crypto_wallets (user_id, currency, address, balance, created_at, updated_at)
            VALUES ($1, $2, $3, 0, NOW(), NOW())
            RETURNING id, user_id, currency as "currency: CryptoCurrency", address, balance, created_at, updated_at
            "#,
            user_id,
            currency as CryptoCurrency,
            address
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(wallet)
    }

    pub async fn process_crypto_payment(
        &self,
        from_user_id: i32,
        to_user_id: i32,
        amount: Decimal,
        currency: CryptoCurrency,
    ) -> Result<CryptoTransaction, ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Get wallets
        let from_wallet = self.get_user_wallet(from_user_id, &currency).await?;
        let to_wallet = self.get_user_wallet(to_user_id, &currency).await?;

        // Check balance
        if from_wallet.balance < amount {
            return Err(ApiError::BadRequest("Insufficient balance".to_string()));
        }

        // Update balances
        sqlx::query!(
            "UPDATE crypto_wallets SET balance = balance - $1 WHERE id = $2",
            amount,
            from_wallet.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        sqlx::query!(
            "UPDATE crypto_wallets SET balance = balance + $1 WHERE id = $2",
            amount,
            to_wallet.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create transaction record
        let tx_hash = self.generate_tx_hash();
        let transaction = sqlx::query_as!(
            CryptoTransaction,
            r#"
            INSERT INTO crypto_transactions (from_wallet_id, to_wallet_id, currency, amount, tx_hash, status, created_at)
            VALUES ($1, $2, $3, $4, $5, 'Confirmed', NOW())
            RETURNING id, from_wallet_id, to_wallet_id, currency as "currency: CryptoCurrency", amount, tx_hash, status as "status: TransactionStatus", created_at
            "#,
            from_wallet.id,
            to_wallet.id,
            currency as CryptoCurrency,
            amount,
            tx_hash
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(transaction)
    }

    pub async fn get_exchange_rate(
        &self,
        from: &str,
        to: &str,
    ) -> Result<ExchangeRate, ApiError> {
        // Mock implementation - integrate with real crypto API like CoinGecko
        let rate = match (from, to) {
            ("BTC", "USD") => Decimal::from(45000),
            ("ETH", "USD") => Decimal::from(2500),
            ("USDT", "USD") => Decimal::from(1),
            ("USDC", "USD") => Decimal::from(1),
            _ => return Err(ApiError::BadRequest("Unsupported currency pair".to_string())),
        };

        Ok(ExchangeRate {
            from_currency: from.to_string(),
            to_currency: to.to_string(),
            rate,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn get_user_wallet(
        &self,
        user_id: i32,
        currency: &CryptoCurrency,
    ) -> Result<CryptoWallet, ApiError> {
        sqlx::query_as!(
            CryptoWallet,
            r#"
            SELECT id, user_id, currency as "currency: CryptoCurrency", address, balance, created_at, updated_at
            FROM crypto_wallets
            WHERE user_id = $1 AND currency = $2
            "#,
            user_id,
            currency as &CryptoCurrency
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Wallet not found".to_string()))
    }

    fn generate_wallet_address(&self, currency: &CryptoCurrency) -> Result<String, ApiError> {
        use rand::Rng;
        let prefix = match currency {
            CryptoCurrency::BTC => "bc1",
            CryptoCurrency::ETH => "0x",
            CryptoCurrency::USDT => "0x",
            CryptoCurrency::USDC => "0x",
        };
        let random: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(40)
            .map(char::from)
            .collect();
        Ok(format!("{}{}", prefix, random))
    }

    fn generate_tx_hash(&self) -> String {
        use rand::Rng;
        let random: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        format!("0x{}", random)
    }
}
