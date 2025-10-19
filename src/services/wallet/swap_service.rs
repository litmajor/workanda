
use sqlx::PgPool;
use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::Utc;
use crate::models::swap::*;
use crate::services::wallet::exchange_rate_service::ExchangeRateService;
use crate::api::error::ApiError;

pub struct SwapService {
    pool: PgPool,
    exchange_rate_service: ExchangeRateService,
}

impl SwapService {
    pub fn new(pool: PgPool) -> Self {
        let exchange_rate_service = ExchangeRateService::new(pool.clone());
        Self { pool, exchange_rate_service }
    }

    /// Get a swap quote
    pub async fn get_swap_quote(
        &self,
        from_currency: &str,
        to_currency: &str,
        amount: Decimal,
    ) -> Result<SwapQuote, ApiError> {
        // Get exchange rate
        let rate = self.exchange_rate_service
            .get_exchange_rate(from_currency, to_currency)
            .await
            .map_err(|e| ApiError::InternalServerError(format!("Failed to get exchange rate: {:?}", e)))?
            .ok_or_else(|| ApiError::NotFound("Exchange rate not found".to_string()))?;

        let rate_decimal = rate.rate;
        let estimated_amount = amount * rate_decimal;

        // Calculate fees (0.3% for DEX swaps, 0.1% for stablecoin pairs)
        let fee_percentage = if Self::is_stablecoin_pair(from_currency, to_currency) {
            Decimal::from_str_exact("0.001").unwrap() // 0.1%
        } else {
            Decimal::from_str_exact("0.003").unwrap() // 0.3%
        };
        let fees = estimated_amount * fee_percentage;

        // Determine provider
        let provider = if Self::is_stablecoin_pair(from_currency, to_currency) {
            SwapProvider::Internal
        } else {
            SwapProvider::Ubeswap
        };

        Ok(SwapQuote {
            from_currency: from_currency.to_string(),
            to_currency: to_currency.to_string(),
            from_amount: amount,
            estimated_to_amount: estimated_amount - fees,
            exchange_rate: rate_decimal,
            price_impact: Decimal::from_str_exact("0.01").unwrap(), // Mock 1% price impact
            fees,
            route: vec![from_currency.to_string(), to_currency.to_string()],
            provider,
            valid_until: Utc::now() + chrono::Duration::minutes(5),
        })
    }

    /// Execute a currency swap
    pub async fn execute_swap(
        &self,
        request: CreateSwapRequest,
    ) -> Result<CurrencySwap, ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Check balance
        let from_balance = sqlx::query_scalar!(
            "SELECT balance FROM wallet_balances WHERE wallet_id = $1 AND currency_code = $2 FOR UPDATE",
            request.wallet_id,
            request.from_currency
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Balance not found".to_string()))?;

        if from_balance < request.amount {
            return Err(ApiError::BadRequest("Insufficient balance".to_string()));
        }

        // Get quote
        let quote = self.get_swap_quote(&request.from_currency, &request.to_currency, request.amount).await?;

        // Check slippage tolerance
        let slippage_tolerance = request.slippage_tolerance.unwrap_or(Decimal::from_str_exact("0.005").unwrap()); // Default 0.5%
        let min_output = quote.estimated_to_amount * (Decimal::ONE - slippage_tolerance);

        // Deduct from source currency
        sqlx::query!(
            "UPDATE wallet_balances SET balance = balance - $1 WHERE wallet_id = $2 AND currency_code = $3",
            request.amount,
            request.wallet_id,
            request.from_currency
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Add to destination currency
        let to_amount = quote.estimated_to_amount;
        sqlx::query!(
            "UPDATE wallet_balances SET balance = balance + $1 WHERE wallet_id = $2 AND currency_code = $3",
            to_amount,
            request.wallet_id,
            request.to_currency
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create swap record
        let swap = sqlx::query_as!(
            CurrencySwap,
            r#"
            INSERT INTO currency_swaps (
                wallet_id, from_currency, to_currency, from_amount, to_amount,
                exchange_rate, slippage_tolerance, actual_slippage, swap_provider,
                fees, status, created_at, completed_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'Completed', NOW(), NOW())
            RETURNING id, wallet_id, from_currency, to_currency, from_amount, to_amount,
                exchange_rate, slippage_tolerance, actual_slippage,
                swap_provider as "swap_provider: SwapProvider", tx_hash,
                fees, status as "status: SwapStatus", created_at, completed_at
            "#,
            request.wallet_id,
            request.from_currency,
            request.to_currency,
            request.amount,
            to_amount,
            quote.exchange_rate,
            slippage_tolerance,
            Some(Decimal::ZERO), // Actual slippage
            quote.provider as SwapProvider,
            quote.fees
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Check auto-conversion preference and trigger if needed
        self.check_auto_conversion(request.wallet_id, &request.to_currency, to_amount).await?;

        Ok(swap)
    }

    /// Set auto-conversion preference
    pub async fn set_auto_conversion(
        &self,
        wallet_id: i32,
        target_currency: String,
        enabled: bool,
        minimum_amount: Decimal,
    ) -> Result<AutoConversionPreference, ApiError> {
        let preference = sqlx::query_as!(
            AutoConversionPreference,
            r#"
            INSERT INTO auto_conversion_preferences (wallet_id, enabled, target_currency, minimum_amount, convert_on_receive, created_at, updated_at)
            VALUES ($1, $2, $3, $4, true, NOW(), NOW())
            ON CONFLICT (wallet_id) DO UPDATE SET
                enabled = $2, target_currency = $3, minimum_amount = $4, updated_at = NOW()
            RETURNING id, wallet_id, enabled, target_currency, minimum_amount, convert_on_receive, created_at, updated_at
            "#,
            wallet_id,
            enabled,
            target_currency,
            minimum_amount
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(preference)
    }

    /// Get swap analytics
    pub async fn get_swap_analytics(&self, wallet_id: i32) -> Result<SwapAnalytics, ApiError> {
        let total_swaps = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM currency_swaps WHERE wallet_id = $1",
            wallet_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(0);

        let total_fees = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(fees), 0) FROM currency_swaps WHERE wallet_id = $1",
            wallet_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(Decimal::ZERO);

        Ok(SwapAnalytics {
            total_swaps,
            total_volume_usd: Decimal::ZERO, // TODO: Calculate
            average_slippage: Decimal::from_str_exact("0.002").unwrap(),
            most_swapped_pairs: vec![],
            total_fees_paid: total_fees,
        })
    }

    async fn check_auto_conversion(
        &self,
        wallet_id: i32,
        received_currency: &str,
        amount: Decimal,
    ) -> Result<(), ApiError> {
        let preference = sqlx::query_as!(
            AutoConversionPreference,
            "SELECT * FROM auto_conversion_preferences WHERE wallet_id = $1 AND enabled = true",
            wallet_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        if let Some(pref) = preference {
            if pref.target_currency != received_currency && amount >= pref.minimum_amount {
                // Auto-convert
                let swap_request = CreateSwapRequest {
                    wallet_id,
                    from_currency: received_currency.to_string(),
                    to_currency: pref.target_currency,
                    amount,
                    slippage_tolerance: Some(Decimal::from_str_exact("0.01").unwrap()), // 1% tolerance
                    auto_approve: true,
                };
                self.execute_swap(swap_request).await?;
            }
        }

        Ok(())
    }

    fn is_stablecoin_pair(from: &str, to: &str) -> bool {
        let stablecoins = ["cUSD", "USDT", "USDC", "cEUR", "cREAL"];
        stablecoins.contains(&from) && stablecoins.contains(&to)
    }
}
