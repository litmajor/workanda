
use sqlx::PgPool;
use rust_decimal::Decimal;
use crate::models::wallet::*;
use crate::api::error::ApiError;
use uuid::Uuid;

pub struct WalletService {
    pool: PgPool,
}

impl WalletService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_wallet(
        &self,
        user_id: i32,
        request: CreateWalletRequest,
    ) -> Result<Wallet, ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Check if user already has a wallet in this currency
        let existing = sqlx::query!(
            "SELECT id FROM wallets WHERE user_id = $1 AND currency = $2",
            user_id,
            request.currency
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        if existing.is_some() {
            return Err(ApiError::BadRequest("Wallet already exists for this currency".to_string()));
        }

        // If this is set as primary, unset other primary wallets
        if request.is_primary {
            sqlx::query!(
                "UPDATE wallets SET is_primary = false WHERE user_id = $1",
                user_id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        }

        let wallet = sqlx::query_as!(
            Wallet,
            r#"
            INSERT INTO wallets (user_id, currency, balance, available_balance, locked_balance, is_primary, status, created_at, updated_at)
            VALUES ($1, $2, 0, 0, 0, $3, 'Active', NOW(), NOW())
            RETURNING id, user_id, currency, balance, available_balance, locked_balance, wallet_address, is_primary, status as "status: WalletStatus", created_at, updated_at
            "#,
            user_id,
            request.currency,
            request.is_primary
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(wallet)
    }

    pub async fn get_user_wallets(&self, user_id: i32) -> Result<Vec<Wallet>, ApiError> {
        let wallets = sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, currency, balance, available_balance, locked_balance, wallet_address, is_primary, status as "status: WalletStatus", created_at, updated_at
            FROM wallets
            WHERE user_id = $1
            ORDER BY is_primary DESC, created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(wallets)
    }

    pub async fn deposit(
        &self,
        user_id: i32,
        request: DepositRequest,
    ) -> Result<WalletTransaction, ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Get wallet and verify ownership
        let wallet = sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, currency, balance, available_balance, locked_balance, wallet_address, is_primary, status as "status: WalletStatus", created_at, updated_at
            FROM wallets
            WHERE id = $1 AND user_id = $2 AND status = 'Active'
            FOR UPDATE
            "#,
            request.wallet_id,
            user_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Wallet not found".to_string()))?;

        let balance_before = wallet.balance;
        let balance_after = balance_before + request.amount;

        // Update wallet balance
        sqlx::query!(
            r#"
            UPDATE wallets
            SET balance = balance + $1,
                available_balance = available_balance + $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            request.amount,
            request.wallet_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create transaction record
        let transaction = sqlx::query_as!(
            WalletTransaction,
            r#"
            INSERT INTO wallet_transactions (wallet_id, transaction_type, amount, currency, description, balance_before, balance_after, status, created_at)
            VALUES ($1, 'Deposit', $2, $3, $4, $5, $6, 'Completed', NOW())
            RETURNING id, wallet_id, transaction_type as "transaction_type: TransactionType", amount, currency, description, reference_id, reference_type, balance_before, balance_after, status as "status: TransactionStatus", created_at
            "#,
            request.wallet_id,
            request.amount,
            wallet.currency,
            request.description.unwrap_or_else(|| format!("Deposit via {}", request.payment_method)),
            balance_before,
            balance_after
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(transaction)
    }

    pub async fn withdraw(
        &self,
        user_id: i32,
        request: WithdrawalRequest,
    ) -> Result<WalletTransaction, ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Get wallet and verify ownership
        let wallet = sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, currency, balance, available_balance, locked_balance, wallet_address, is_primary, status as "status: WalletStatus", created_at, updated_at
            FROM wallets
            WHERE id = $1 AND user_id = $2 AND status = 'Active'
            FOR UPDATE
            "#,
            request.wallet_id,
            user_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Wallet not found".to_string()))?;

        // Check available balance
        if wallet.available_balance < request.amount {
            return Err(ApiError::BadRequest("Insufficient available balance".to_string()));
        }

        let balance_before = wallet.balance;
        let balance_after = balance_before - request.amount;

        // Update wallet balance
        sqlx::query!(
            r#"
            UPDATE wallets
            SET balance = balance - $1,
                available_balance = available_balance - $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            request.amount,
            request.wallet_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create transaction record
        let transaction = sqlx::query_as!(
            WalletTransaction,
            r#"
            INSERT INTO wallet_transactions (wallet_id, transaction_type, amount, currency, description, balance_before, balance_after, status, created_at)
            VALUES ($1, 'Withdrawal', $2, $3, $4, $5, $6, 'Pending', NOW())
            RETURNING id, wallet_id, transaction_type as "transaction_type: TransactionType", amount, currency, description, reference_id, reference_type, balance_before, balance_after, status as "status: TransactionStatus", created_at
            "#,
            request.wallet_id,
            request.amount,
            wallet.currency,
            request.description.unwrap_or_else(|| format!("Withdrawal to {}", request.destination)),
            balance_before,
            balance_after
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(transaction)
    }

    pub async fn transfer(
        &self,
        from_user_id: i32,
        request: TransferRequest,
    ) -> Result<(WalletTransaction, WalletTransaction), ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Get sender's wallet
        let from_wallet = sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, currency, balance, available_balance, locked_balance, wallet_address, is_primary, status as "status: WalletStatus", created_at, updated_at
            FROM wallets
            WHERE id = $1 AND user_id = $2 AND status = 'Active'
            FOR UPDATE
            "#,
            request.from_wallet_id,
            from_user_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Source wallet not found".to_string()))?;

        // Check balance
        if from_wallet.available_balance < request.amount {
            return Err(ApiError::BadRequest("Insufficient balance".to_string()));
        }

        // Get or create recipient's wallet
        let to_wallet = match sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, currency, balance, available_balance, locked_balance, wallet_address, is_primary, status as "status: WalletStatus", created_at, updated_at
            FROM wallets
            WHERE user_id = $1 AND currency = $2 AND status = 'Active'
            FOR UPDATE
            "#,
            request.to_user_id,
            request.currency
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))? {
            Some(wallet) => wallet,
            None => {
                // Create wallet for recipient
                sqlx::query_as!(
                    Wallet,
                    r#"
                    INSERT INTO wallets (user_id, currency, balance, available_balance, locked_balance, is_primary, status, created_at, updated_at)
                    VALUES ($1, $2, 0, 0, 0, false, 'Active', NOW(), NOW())
                    RETURNING id, user_id, currency, balance, available_balance, locked_balance, wallet_address, is_primary, status as "status: WalletStatus", created_at, updated_at
                    "#,
                    request.to_user_id,
                    request.currency
                )
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| ApiError::InternalServerError(e.to_string()))?
            }
        };

        // Update sender's wallet
        let from_balance_before = from_wallet.balance;
        let from_balance_after = from_balance_before - request.amount;

        sqlx::query!(
            r#"
            UPDATE wallets
            SET balance = balance - $1,
                available_balance = available_balance - $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            request.amount,
            from_wallet.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Update recipient's wallet
        let to_balance_before = to_wallet.balance;
        let to_balance_after = to_balance_before + request.amount;

        sqlx::query!(
            r#"
            UPDATE wallets
            SET balance = balance + $1,
                available_balance = available_balance + $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
            request.amount,
            to_wallet.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        let description = request.description.unwrap_or_else(|| "Transfer".to_string());

        // Create sender transaction
        let from_tx = sqlx::query_as!(
            WalletTransaction,
            r#"
            INSERT INTO wallet_transactions (wallet_id, transaction_type, amount, currency, description, reference_id, reference_type, balance_before, balance_after, status, created_at)
            VALUES ($1, 'Transfer', $2, $3, $4, $5, 'transfer_out', $6, $7, 'Completed', NOW())
            RETURNING id, wallet_id, transaction_type as "transaction_type: TransactionType", amount, currency, description, reference_id, reference_type, balance_before, balance_after, status as "status: TransactionStatus", created_at
            "#,
            from_wallet.id,
            request.amount,
            request.currency,
            format!("Transfer to user #{}: {}", request.to_user_id, description),
            to_wallet.id.to_string(),
            from_balance_before,
            from_balance_after
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create recipient transaction
        let to_tx = sqlx::query_as!(
            WalletTransaction,
            r#"
            INSERT INTO wallet_transactions (wallet_id, transaction_type, amount, currency, description, reference_id, reference_type, balance_before, balance_after, status, created_at)
            VALUES ($1, 'Transfer', $2, $3, $4, $5, 'transfer_in', $6, $7, 'Completed', NOW())
            RETURNING id, wallet_id, transaction_type as "transaction_type: TransactionType", amount, currency, description, reference_id, reference_type, balance_before, balance_after, status as "status: TransactionStatus", created_at
            "#,
            to_wallet.id,
            request.amount,
            request.currency,
            format!("Transfer from user #{}: {}", from_user_id, description),
            from_wallet.id.to_string(),
            to_balance_before,
            to_balance_after
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok((from_tx, to_tx))
    }

    pub async fn get_wallet_overview(&self, user_id: i32) -> Result<WalletOverview, ApiError> {
        let wallets = self.get_user_wallets(user_id).await?;
        
        let wallet_balances: Vec<WalletBalance> = wallets.iter().map(|w| WalletBalance {
            currency: w.currency.clone(),
            total_balance: w.balance,
            available_balance: w.available_balance,
            locked_balance: w.locked_balance,
        }).collect();

        // Get recent transactions
        let recent_transactions = if let Some(primary_wallet) = wallets.iter().find(|w| w.is_primary) {
            sqlx::query_as!(
                WalletTransaction,
                r#"
                SELECT id, wallet_id, transaction_type as "transaction_type: TransactionType", amount, currency, description, reference_id, reference_type, balance_before, balance_after, status as "status: TransactionStatus", created_at
                FROM wallet_transactions
                WHERE wallet_id = $1
                ORDER BY created_at DESC
                LIMIT 10
                "#,
                primary_wallet.id
            )
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
        } else {
            vec![]
        };

        // Calculate total balance in USD (simplified - you'd use exchange rates)
        let total_balance_usd = wallets.iter()
            .map(|w| w.balance)
            .sum();

        // Get pending escrows
        let pending_escrows = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(amount), 0) as "pending: Decimal"
            FROM escrow_accounts
            WHERE (client_id = $1 OR freelancer_id = $1) AND status = 'LOCKED'
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap_or(Decimal::ZERO);

        Ok(WalletOverview {
            total_balance_usd,
            wallets: wallet_balances,
            recent_transactions,
            pending_escrows,
        })
    }

    pub async fn get_transaction_history(
        &self,
        wallet_id: i32,
        user_id: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<WalletTransaction>, ApiError> {
        // Verify wallet ownership
        sqlx::query!(
            "SELECT id FROM wallets WHERE id = $1 AND user_id = $2",
            wallet_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Wallet not found".to_string()))?;

        let transactions = sqlx::query_as!(
            WalletTransaction,
            r#"
            SELECT id, wallet_id, transaction_type as "transaction_type: TransactionType", amount, currency, description, reference_id, reference_type, balance_before, balance_after, status as "status: TransactionStatus", created_at
            FROM wallet_transactions
            WHERE wallet_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            wallet_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(transactions)
    }
}
