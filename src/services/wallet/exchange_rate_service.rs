use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use reqwest;
use std::collections::HashMap;
use crate::services::wallet::wallet_service::WalletError;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStrExact;

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

#[derive(Debug)]
pub enum ApiError {
    ReqwestError(reqwest::Error),
    IoError(std::io::Error),
    DatabaseError(String),
    ParseError(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::ReqwestError(err)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::IoError(err)
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> Self {
        ApiError::DatabaseError(err)
    }
}

pub struct ExchangeRateService {
    pool: PgPool,
}

impl ExchangeRateService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn update_rate(
        &self,
        base: &str,
        quote: &str,
        rate: rust_decimal::Decimal,
        source: &str,
    ) -> Result<(), ApiError> {
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
        .map_err(|e| ApiError::DatabaseError(format!("Failed to update exchange rate: {}", e)))?;

        Ok(())
    }

    pub async fn get_exchange_rate(
        &self,
        base: &str,
        quote: &str,
    ) -> Result<Option<ExchangeRate>, ApiError> {
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
        .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch exchange rate: {}", e)))?;

        Ok(rate)
    }

    pub async fn convert_currency(
        &self,
        from_currency: &str,
        to_currency: &str,
        amount: f64,
    ) -> Result<ConversionResult, ApiError> {
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
    ) -> Result<f64, ApiError> {
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

        self.update_rate(from, to, rate_decimal, "mock_api").await?;

        Ok(rate)
    }

    pub async fn get_all_rates(&self) -> Result<Vec<ExchangeRate>, ApiError> {
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
        .map_err(|e| ApiError::DatabaseError(format!("Failed to fetch rates: {}", e)))?;

        Ok(rates)
    }

    pub async fn refresh_all_rates(&self) -> Result<(), ApiError> {
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

    /// Fetch current exchange rates from CoinGecko API
    pub async fn fetch_current_rates(&self) -> Result<(), ApiError> {
        // CoinGecko free API endpoint
        let coingecko_api = "https://api.coingecko.com/api/v3/simple/price";

        // Fetch crypto prices
        let crypto_ids = "celo,bitcoin,ethereum,tether,usd-coin";
        let vs_currencies = "usd,eur";

        let url = format!(
            "{}?ids={}&vs_currencies={}",
            coingecko_api, crypto_ids, vs_currencies
        );

        match reqwest::get(&url).await {
            Ok(response) => {
                if let Ok(data) = response.json::<serde_json::Value>().await {
                    // Parse CELO prices
                    if let Some(celo) = data.get("celo") {
                        if let Some(usd) = celo.get("usd").and_then(|v| v.as_f64()) {
                            self.update_rate(
                                "CELO",
                                "USD",
                                Decimal::from_f64_retain(usd).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;

                            self.update_rate(
                                "CELO",
                                "cUSD",
                                Decimal::from_f64_retain(usd).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;
                        }

                        if let Some(eur) = celo.get("eur").and_then(|v| v.as_f64()) {
                            self.update_rate(
                                "CELO",
                                "EUR",
                                Decimal::from_f64_retain(eur).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;

                            self.update_rate(
                                "CELO",
                                "cEUR",
                                Decimal::from_f64_retain(eur).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;
                        }
                    }

                    // Parse BTC prices
                    if let Some(btc) = data.get("bitcoin") {
                        if let Some(usd) = btc.get("usd").and_then(|v| v.as_f64()) {
                            self.update_rate(
                                "BTC",
                                "USD",
                                Decimal::from_f64_retain(usd).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;
                        }
                    }

                    // Parse ETH prices
                    if let Some(eth) = data.get("ethereum") {
                        if let Some(usd) = eth.get("usd").and_then(|v| v.as_f64()) {
                            self.update_rate(
                                "ETH",
                                "USD",
                                Decimal::from_f64_retain(usd).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;
                        }
                    }

                    // Parse USDT prices
                    if let Some(usdt) = data.get("tether") {
                        if let Some(usd) = usdt.get("usd").and_then(|v| v.as_f64()) {
                            self.update_rate(
                                "USDT",
                                "USD",
                                Decimal::from_f64_retain(usd).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;
                        }
                    }

                    // Parse USDC prices
                    if let Some(usdc) = data.get("usd-coin") {
                        if let Some(usd) = usdc.get("usd").and_then(|v| v.as_f64()) {
                            self.update_rate(
                                "USDC",
                                "USD",
                                Decimal::from_f64_retain(usd).unwrap_or(Decimal::ZERO),
                                "coingecko"
                            ).await?;
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch exchange rates from CoinGecko: {}", e);
                // Fall back to static rates if API fails
                return self.fetch_fallback_rates().await;
            }
        }

        // Add stablecoin pegs (1:1 ratios)
        self.update_rate("USD", "cUSD", Decimal::ONE, "peg").await?;
        self.update_rate("EUR", "cEUR", Decimal::ONE, "peg").await?;
        self.update_rate("cUSD", "USDT", Decimal::ONE, "peg").await?;
        self.update_rate("cUSD", "USDC", Decimal::ONE, "peg").await?;

        // Add USD/EUR rate
        self.update_rate("USD", "EUR", Decimal::from_str_exact("0.92").unwrap(), "ecb").await?;

        Ok(())
    }

    /// Fallback rates if API is unavailable
    async fn fetch_fallback_rates(&self) -> Result<(), ApiError> {
        let rates = vec![
            ("USD", "cUSD", Decimal::from_str_exact("1.0").unwrap()),
            ("EUR", "cEUR", Decimal::from_str_exact("1.0").unwrap()),
            ("USD", "EUR", Decimal::from_str_exact("0.92").unwrap()),
            ("USD", "CELO", Decimal::from_str_exact("0.65").unwrap()),
            ("cUSD", "CELO", Decimal::from_str_exact("0.65").unwrap()),
            ("BTC", "USD", Decimal::from_str_exact("45000.0").unwrap()),
            ("ETH", "USD", Decimal::from_str_exact("2500.0").unwrap()),
            ("USDT", "USD", Decimal::from_str_exact("1.0").unwrap()),
            ("USDC", "USD", Decimal::from_str_exact("1.0").unwrap()),
            ("cUSD", "cEUR", Decimal::from_str_exact("0.92").unwrap()),
            ("CELO", "USD", Decimal::from_str_exact("1.54").unwrap()),
            ("CELO", "cUSD", Decimal::from_str_exact("1.54").unwrap()),
        ];

        for (base, quote, rate) in rates {
            self.update_rate(base, quote, rate, "fallback").await?;
        }

        Ok(())
    }
}