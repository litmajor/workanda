
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
use sqlx::PgPool;
use rust_decimal::Decimal;
use crate::models::wallet::*;
use crate::api::error::ApiError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;

pub struct WalletService {
    pool: PgPool,
    encryption_key: Vec<u8>,
}

impl WalletService {
    pub fn new(pool: PgPool, encryption_key: Vec<u8>) -> Self {
        Self { pool, encryption_key }
    }

    /// Create a new wallet for a user
    pub async fn create_wallet(
        &self,
        user_id: i32,
        wallet_type: WalletType,
    ) -> Result<Wallet, ApiError> {
        // Generate Celo-compatible address (0x...)
        let (address, private_key, mnemonic) = self.generate_celo_wallet()?;
        
        // Encrypt private key and mnemonic
        let encrypted_pk = self.encrypt_data(&private_key)?;
        let encrypted_mnemonic = self.encrypt_data(&mnemonic)?;

        let wallet = sqlx::query_as!(
            Wallet,
            r#"
            INSERT INTO wallets (user_id, celo_address, encrypted_private_key, mnemonic_encrypted, wallet_type, is_verified, created_at)
            VALUES ($1, $2, $3, $4, $5, false, NOW())
            RETURNING id, user_id, celo_address, encrypted_private_key, mnemonic_encrypted, 
                      wallet_type as "wallet_type: WalletType", is_verified, created_at
            "#,
            user_id,
            address,
            encrypted_pk,
            Some(encrypted_mnemonic),
            wallet_type as WalletType
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Initialize balances for major currencies
        self.initialize_balances(wallet.id).await?;

        Ok(wallet)
    }

    /// Get wallet by user ID
    pub async fn get_wallet_by_user(
        &self,
        user_id: i32,
    ) -> Result<Wallet, ApiError> {
        sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, celo_address, encrypted_private_key, mnemonic_encrypted,
                   wallet_type as "wallet_type: WalletType", is_verified, created_at
            FROM wallets
            WHERE user_id = $1 AND wallet_type = 'Individual'
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Wallet not found".to_string()))
    }

    /// Get wallet balances
    pub async fn get_balances(
        &self,
        wallet_id: i32,
    ) -> Result<Vec<WalletBalance>, ApiError> {
        sqlx::query_as!(
            WalletBalance,
            "SELECT * FROM wallet_balances WHERE wallet_id = $1 ORDER BY currency_code",
            wallet_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))
    }

    /// Send money to another wallet or address
    pub async fn send_money(
        &self,
        request: SendMoneyRequest,
    ) -> Result<WalletTransaction, ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Get sender wallet
        let sender = self.get_wallet(request.from_wallet_id).await?;
        
        // Check balance
        let balance = self.get_balance(request.from_wallet_id, &request.currency_code).await?;
        if balance.balance < request.amount {
            return Err(ApiError::BadRequest("Insufficient balance".to_string()));
        }

        // Determine if recipient is Workanda user or external address
        let to_wallet_id = if request.to_address.starts_with("0x") {
            None // External address
        } else {
            // Try to find user by username
            Some(self.find_wallet_by_username(&request.to_address).await?)
        };

        // Update sender balance
        sqlx::query!(
            "UPDATE wallet_balances SET balance = balance - $1 WHERE wallet_id = $2 AND currency_code = $3",
            request.amount,
            request.from_wallet_id,
            request.currency_code
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Update recipient balance if internal
        if let Some(to_id) = to_wallet_id {
            sqlx::query!(
                "UPDATE wallet_balances SET balance = balance + $1 WHERE wallet_id = $2 AND currency_code = $3",
                request.amount,
                to_id,
                request.currency_code
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        }

        // Create transaction record
        let transaction = sqlx::query_as!(
            WalletTransaction,
            r#"
            INSERT INTO transactions (from_wallet_id, to_wallet_id, to_address, amount, currency_code, status, tx_type, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, 'Pending', 'Payment', $6, NOW())
            RETURNING id, from_wallet_id, to_wallet_id, to_address, amount, currency_code, tx_hash,
                      status as "status: TransactionStatus", tx_type as "tx_type: TransactionType",
                      gas_fee, metadata, created_at, confirmed_at
            "#,
            Some(request.from_wallet_id),
            to_wallet_id,
            if to_wallet_id.is_none() { Some(request.to_address.clone()) } else { None },
            request.amount,
            request.currency_code,
            request.metadata
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // If external address, submit to Celo blockchain
        if to_wallet_id.is_none() {
            // TODO: Integrate with Celo service to broadcast transaction
            // let tx_hash = celo_service.send_transaction(...).await?;
        }

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(transaction)
    }

    /// Get transaction history
    pub async fn get_transactions(
        &self,
        wallet_id: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<WalletTransaction>, ApiError> {
        sqlx::query_as!(
            WalletTransaction,
            r#"
            SELECT id, from_wallet_id, to_wallet_id, to_address, amount, currency_code, tx_hash,
                   status as "status: TransactionStatus", tx_type as "tx_type: TransactionType",
                   gas_fee, metadata, created_at, confirmed_at
            FROM transactions
            WHERE from_wallet_id = $1 OR to_wallet_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            wallet_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))
    }

    /// Get wallet statistics
    pub async fn get_wallet_stats(
        &self,
        wallet_id: i32,
    ) -> Result<WalletStats, ApiError> {
        let balances = self.get_balances(wallet_id).await?;
        
        // Calculate total in USD
        let mut total_usd = Decimal::ZERO;
        for balance in &balances {
            // TODO: Get exchange rate and convert
            total_usd += balance.balance;
        }

        // Get pending transactions count
        let pending = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM transactions WHERE (from_wallet_id = $1 OR to_wallet_id = $1) AND status = 'Pending'",
            wallet_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(0) as i32;

        Ok(WalletStats {
            total_balance_usd: total_usd,
            balances,
            pending_transactions: pending,
            monthly_income: Decimal::ZERO, // TODO: Calculate
            monthly_expenses: Decimal::ZERO, // TODO: Calculate
        })
    }

    // Helper methods

    async fn initialize_balances(&self, wallet_id: i32) -> Result<(), ApiError> {
        let currencies = vec!["USD", "cUSD", "cEUR", "CELO", "BTC", "ETH"];
        
        for currency in currencies {
            sqlx::query!(
                "INSERT INTO wallet_balances (wallet_id, currency_code, balance, locked_balance, updated_at) VALUES ($1, $2, 0, 0, NOW())",
                wallet_id,
                currency
            )
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        }

        Ok(())
    }

    async fn get_wallet(&self, wallet_id: i32) -> Result<Wallet, ApiError> {
        sqlx::query_as!(
            Wallet,
            r#"
            SELECT id, user_id, celo_address, encrypted_private_key, mnemonic_encrypted,
                   wallet_type as "wallet_type: WalletType", is_verified, created_at
            FROM wallets WHERE id = $1
            "#,
            wallet_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Wallet not found".to_string()))
    }

    async fn get_balance(&self, wallet_id: i32, currency: &str) -> Result<WalletBalance, ApiError> {
        sqlx::query_as!(
            WalletBalance,
            "SELECT * FROM wallet_balances WHERE wallet_id = $1 AND currency_code = $2",
            wallet_id,
            currency
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Balance not found".to_string()))
    }

    async fn find_wallet_by_username(&self, username: &str) -> Result<i32, ApiError> {
        // TODO: Join with users table to find wallet by username
        Err(ApiError::NotFound("User not found".to_string()))
    }

    fn generate_celo_wallet(&self) -> Result<(String, String, String), ApiError> {
        // TODO: Implement proper Celo wallet generation
        // This is a placeholder - use ethers-rs or web3 library
        let address = format!("0x{}", Self::random_hex(40));
        let private_key = Self::random_hex(64);
        let mnemonic = "word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12".to_string();
        
        Ok((address, private_key, mnemonic))
    }

    fn random_hex(len: usize) -> String {
        use rand::distributions::Alphanumeric;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    fn encrypt_data(&self, data: &str) -> Result<String, ApiError> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|_| ApiError::InternalServerError("Encryption error".to_string()))?;
        
        let nonce = Nonce::from_slice(b"unique nonce");
        let ciphertext = cipher.encrypt(nonce, data.as_bytes())
            .map_err(|_| ApiError::InternalServerError("Encryption error".to_string()))?;
        
        Ok(hex::encode(ciphertext))
    }
}
