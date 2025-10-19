
use sqlx::PgPool;
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::models::fiat_gateway::*;
use crate::models::kyc::{KycVerification, KycLevel};
use crate::api::error::ApiError;

pub struct FiatGatewayService {
    pool: PgPool,
}

impl FiatGatewayService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Deposit fiat and convert to crypto
    pub async fn deposit_fiat(
        &self,
        request: DepositFiatRequest,
    ) -> Result<FiatTransaction, ApiError> {
        // Check KYC limits
        self.check_kyc_limits(request.wallet_id, request.amount, true).await?;

        // Process payment through provider
        let provider_response = match request.payment_provider {
            PaymentProvider::Stripe => self.process_stripe_payment(&request).await?,
            PaymentProvider::Flutterwave => self.process_flutterwave_payment(&request).await?,
            PaymentProvider::Paystack => self.process_paystack_payment(&request).await?,
            PaymentProvider::MPesa => self.process_mpesa_payment(&request).await?,
            PaymentProvider::MTNMobileMoney => self.process_mtn_payment(&request).await?,
            PaymentProvider::AirtelMoney => self.process_airtel_payment(&request).await?,
        };

        // Calculate fees (2.5% for card, 1% for mobile money)
        let fee_percentage = match request.payment_method {
            PaymentMethod::CreditCard | PaymentMethod::DebitCard => Decimal::from_str_exact("0.025").unwrap(),
            PaymentMethod::MobileMoney => Decimal::from_str_exact("0.01").unwrap(),
            PaymentMethod::BankTransfer => Decimal::from_str_exact("0.005").unwrap(),
        };
        let fees = request.amount * fee_percentage;

        // Create transaction record
        let transaction = sqlx::query_as!(
            FiatTransaction,
            r#"
            INSERT INTO fiat_transactions (
                wallet_id, transaction_type, amount, currency_code, payment_method, 
                payment_provider, provider_transaction_id, status, fees, metadata, created_at
            )
            VALUES ($1, 'Deposit', $2, $3, $4, $5, $6, 'Processing', $7, $8, NOW())
            RETURNING id, wallet_id, transaction_type as "transaction_type: FiatTransactionType",
                amount, currency_code, payment_method as "payment_method: PaymentMethod",
                payment_provider as "payment_provider: PaymentProvider",
                provider_transaction_id, status as "status: FiatTransactionStatus",
                fees, destination_address, metadata, created_at, completed_at
            "#,
            request.wallet_id,
            request.amount,
            request.currency_code,
            request.payment_method as PaymentMethod,
            request.payment_provider as PaymentProvider,
            Some(provider_response.transaction_id),
            fees,
            None::<serde_json::Value>
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(transaction)
    }

    /// Withdraw crypto and convert to fiat
    pub async fn withdraw_fiat(
        &self,
        request: WithdrawFiatRequest,
    ) -> Result<FiatTransaction, ApiError> {
        // Check KYC limits
        self.check_kyc_limits(request.wallet_id, request.amount, false).await?;

        // Check crypto balance
        let balance = sqlx::query_scalar!(
            "SELECT balance FROM wallet_balances WHERE wallet_id = $1 AND currency_code = $2",
            request.wallet_id,
            request.currency_code
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Balance not found".to_string()))?;

        if balance < request.amount {
            return Err(ApiError::BadRequest("Insufficient balance".to_string()));
        }

        // Calculate fees
        let fees = request.amount * Decimal::from_str_exact("0.01").unwrap(); // 1% withdrawal fee

        // Process withdrawal through provider
        let provider_response = match request.payment_provider {
            PaymentProvider::Flutterwave => self.process_flutterwave_withdrawal(&request).await?,
            PaymentProvider::Paystack => self.process_paystack_withdrawal(&request).await?,
            PaymentProvider::MPesa => self.process_mpesa_withdrawal(&request).await?,
            _ => return Err(ApiError::BadRequest("Provider not supported for withdrawals".to_string())),
        };

        // Create transaction record
        let transaction = sqlx::query_as!(
            FiatTransaction,
            r#"
            INSERT INTO fiat_transactions (
                wallet_id, transaction_type, amount, currency_code, payment_method,
                payment_provider, provider_transaction_id, status, fees, destination_address, created_at
            )
            VALUES ($1, 'Withdrawal', $2, $3, $4, $5, $6, 'Processing', $7, $8, NOW())
            RETURNING id, wallet_id, transaction_type as "transaction_type: FiatTransactionType",
                amount, currency_code, payment_method as "payment_method: PaymentMethod",
                payment_provider as "payment_provider: PaymentProvider",
                provider_transaction_id, status as "status: FiatTransactionStatus",
                fees, destination_address, metadata, created_at, completed_at
            "#,
            request.wallet_id,
            request.amount,
            request.currency_code,
            request.payment_method as PaymentMethod,
            request.payment_provider as PaymentProvider,
            Some(provider_response.transaction_id),
            fees,
            request.mobile_money_number.or(request.bank_account.as_ref().map(|b| b.account_number.clone()))
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Lock balance
        sqlx::query!(
            "UPDATE wallet_balances SET balance = balance - $1, locked_balance = locked_balance + $1 WHERE wallet_id = $2 AND currency_code = $3",
            request.amount,
            request.wallet_id,
            request.currency_code
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(transaction)
    }

    async fn check_kyc_limits(
        &self,
        wallet_id: i32,
        amount: Decimal,
        is_deposit: bool,
    ) -> Result<(), ApiError> {
        // Get user's KYC level
        let kyc = sqlx::query_as!(
            KycVerification,
            r#"
            SELECT kv.* FROM kyc_verifications kv
            JOIN wallets w ON w.user_id = kv.user_id
            WHERE w.id = $1 AND kv.status = 'Approved'
            ORDER BY kv.verification_level DESC
            LIMIT 1
            "#,
            wallet_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        let kyc_level = kyc.as_ref().map(|k| k.verification_level.clone()).unwrap_or(KycLevel::None);
        let limits = kyc_level.get_limits();

        let limit = if is_deposit {
            limits.daily_deposit_limit
        } else {
            limits.daily_withdrawal_limit
        };

        if amount > limit {
            return Err(ApiError::BadRequest(format!(
                "Amount exceeds daily limit of {} for KYC level {:?}",
                limit, kyc_level
            )));
        }

        Ok(())
    }

    // Payment provider integrations (mock implementations)
    async fn process_stripe_payment(&self, _request: &DepositFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with Stripe API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("stripe_{}", Uuid::new_v4()),
            payment_url: Some("https://checkout.stripe.com/...".to_string()),
            message: Some("Payment initiated".to_string()),
        })
    }

    async fn process_flutterwave_payment(&self, _request: &DepositFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with Flutterwave API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("flw_{}", Uuid::new_v4()),
            payment_url: Some("https://checkout.flutterwave.com/...".to_string()),
            message: Some("Payment initiated".to_string()),
        })
    }

    async fn process_paystack_payment(&self, _request: &DepositFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with Paystack API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("ps_{}", Uuid::new_v4()),
            payment_url: Some("https://checkout.paystack.com/...".to_string()),
            message: Some("Payment initiated".to_string()),
        })
    }

    async fn process_mpesa_payment(&self, _request: &DepositFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with M-Pesa API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("mpesa_{}", Uuid::new_v4()),
            payment_url: None,
            message: Some("STK push sent".to_string()),
        })
    }

    async fn process_mtn_payment(&self, _request: &DepositFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with MTN Mobile Money API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("mtn_{}", Uuid::new_v4()),
            payment_url: None,
            message: Some("Payment request sent".to_string()),
        })
    }

    async fn process_airtel_payment(&self, _request: &DepositFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with Airtel Money API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("airtel_{}", Uuid::new_v4()),
            payment_url: None,
            message: Some("Payment request sent".to_string()),
        })
    }

    async fn process_flutterwave_withdrawal(&self, _request: &WithdrawFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with Flutterwave Payouts API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("flw_payout_{}", Uuid::new_v4()),
            payment_url: None,
            message: Some("Payout initiated".to_string()),
        })
    }

    async fn process_paystack_withdrawal(&self, _request: &WithdrawFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with Paystack Transfer API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("ps_transfer_{}", Uuid::new_v4()),
            payment_url: None,
            message: Some("Transfer initiated".to_string()),
        })
    }

    async fn process_mpesa_withdrawal(&self, _request: &WithdrawFiatRequest) -> Result<PaymentProviderResponse, ApiError> {
        // TODO: Integrate with M-Pesa B2C API
        Ok(PaymentProviderResponse {
            success: true,
            transaction_id: format!("mpesa_b2c_{}", Uuid::new_v4()),
            payment_url: None,
            message: Some("Withdrawal initiated".to_string()),
        })
    }
}
