use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use reqwest;
use std::collections::HashMap;
use crate::services::wallet::wallet_service::WalletError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExchangeRate {
    pub id: Uuid,
    pub base_currency: String,
    pub quote_currency: String,
    pub rate: rust_decimal::Decimal,
    pub source: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount: f64,
    pub to_amount: f64,
    pub rate: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct ExchangeRateService {
    pool: PgPool,
}

impl ExchangeRateService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn update_exchange_rate(
        &self,
        base: &str,
        quote: &str,
        rate: rust_decimal::Decimal,
        source: &str,
    ) -> Result<(), WalletError> {
        sqlx::query!(
            r#"
            INSERT INTO exchange_rates (base_currency, quote_currency, rate, source)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (base_currency, quote_currency, source)
            DO UPDATE SET rate = $3, updated_at = CURRENT_TIMESTAMP
            "#,
            base,
            quote,
            rate,
            source
        )
        .execute(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to update exchange rate: {}", e)))?;

        Ok(())
    }

    pub async fn get_exchange_rate(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<Option<ExchangeRate>, WalletError> {
        let rate = sqlx::query_as!(
            ExchangeRate,
            r#"
            SELECT id, base_currency, quote_currency, rate, source, updated_at
            FROM exchange_rates
            WHERE base_currency = $1 AND quote_currency = $2
            ORDER BY updated_at DESC
            LIMIT 1
            "#,
            base,
            quote
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch exchange rate: {}", e)))?;

        Ok(rate)
    }

    pub async fn convert_currency(
        &self,
        from_currency: &str,
        to_currency: &str,
        amount: f64,
    ) -> Result<ConversionResult, WalletError> {
        if from_currency == to_currency {
            return Ok(ConversionResult {
                from_currency: from_currency.to_string(),
                to_currency: to_currency.to_string(),
                from_amount: amount,
                to_amount: amount,
                rate: 1.0,
                timestamp: Utc::now(),
            });
        }

        let rate_record = self.get_exchange_rate(from_currency, to_currency).await?;
        
        let rate = match rate_record {
            Some(r) => r.rate.to_string().parse::<f64>().unwrap_or(1.0),
            None => {
                self.fetch_and_cache_rate(from_currency, to_currency).await?
            }
        };

        let converted_amount = amount * rate;

        Ok(ConversionResult {
            from_currency: from_currency.to_string(),
            to_currency: to_currency.to_string(),
            from_amount: amount,
            to_amount: converted_amount,
            rate,
            timestamp: Utc::now(),
        })
    }

    async fn fetch_and_cache_rate(
        &self,
        from: &str,
        to: &str,
    ) -> Result<f64, WalletError> {
        let mock_rates: HashMap<(&str, &str), f64> = [
            (("USD", "EUR"), 0.92),
            (("USD", "GBP"), 0.79),
            (("USD", "NGN"), 1620.0),
            (("USD", "KES"), 129.5),
            (("USD", "ZAR"), 18.5),
            (("USD", "GHS"), 15.8),
            (("USD", "UGX"), 3700.0),
            (("cUSD", "USD"), 1.0),
            (("cEUR", "EUR"), 1.0),
            (("cREAL", "BRL"), 1.0),
            (("CELO", "USD"), 0.65),
            (("BTC", "USD"), 67000.0),
            (("ETH", "USD"), 3500.0),
            (("USDT", "USD"), 1.0),
            (("USDC", "USD"), 1.0),
        ]
        .iter()
        .cloned()
        .collect();

        let rate = mock_rates.get(&(from, to))
            .or_else(|| {
                mock_rates.get(&(to, from)).map(|r| &(1.0 / r))
            })
            .copied()
            .unwrap_or(1.0);

        let rate_decimal = rust_decimal::Decimal::from_f64_retain(rate)
            .unwrap_or(rust_decimal::Decimal::new(1, 0));

        self.update_exchange_rate(from, to, rate_decimal, "mock_api").await?;

        Ok(rate)
    }

    pub async fn get_all_rates(&self) -> Result<Vec<ExchangeRate>, WalletError> {
        let rates = sqlx::query_as!(
            ExchangeRate,
            r#"
            SELECT DISTINCT ON (base_currency, quote_currency)
                id, base_currency, quote_currency, rate, source, updated_at
            FROM exchange_rates
            ORDER BY base_currency, quote_currency, updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch rates: {}", e)))?;

        Ok(rates)
    }

    pub async fn refresh_all_rates(&self) -> Result<(), WalletError> {
        let currency_pairs = vec![
            ("USD", "EUR"), ("USD", "GBP"), ("USD", "NGN"), ("USD", "KES"),
            ("USD", "ZAR"), ("USD", "GHS"), ("USD", "UGX"),
            ("cUSD", "USD"), ("cEUR", "EUR"), ("cREAL", "BRL"),
            ("CELO", "USD"), ("BTC", "USD"), ("ETH", "USD"),
            ("USDT", "USD"), ("USDC", "USD"),
        ];

        for (base, quote) in currency_pairs {
            self.fetch_and_cache_rate(base, quote).await?;
        }

        Ok(())
    }
}
