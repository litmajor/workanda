
use sqlx::PgPool;
use rust_decimal::Decimal;
use crate::models::wallet_escrow::*;
use crate::api::error::ApiError;
use crate::services::telegram_bot_service::TelegramBotService;
use crate::services::email_service::EmailService;

pub struct WalletEscrowService {
    pool: PgPool,
    telegram_service: TelegramBotService,
    email_service: EmailService,
}

impl WalletEscrowService {
    pub fn new(pool: PgPool) -> Self {
        let telegram_service = TelegramBotService::new();
        let email_service = EmailService::new();
        Self { pool, telegram_service, email_service }
    }

    /// Create escrow from wallet
    pub async fn create_escrow(
        &self,
        request: CreateEscrowRequest,
    ) -> Result<WalletEscrow, ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Check wallet balance
        let balance = sqlx::query_scalar!(
            "SELECT balance FROM wallet_balances WHERE wallet_id = $1 AND currency_code = $2 FOR UPDATE",
            request.wallet_id,
            request.currency_code
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Wallet balance not found".to_string()))?;

        if balance < request.amount {
            return Err(ApiError::BadRequest("Insufficient balance for escrow".to_string()));
        }

        // Lock funds in wallet
        sqlx::query!(
            "UPDATE wallet_balances SET balance = balance - $1, locked_balance = locked_balance + $1 WHERE wallet_id = $2 AND currency_code = $3",
            request.amount,
            request.wallet_id,
            request.currency_code
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create escrow record
        let escrow = sqlx::query_as!(
            WalletEscrow,
            r#"
            INSERT INTO wallet_escrows (
                wallet_id, project_id, contract_id, amount, currency_code, status, locked_at, created_at
            )
            VALUES ($1, $2, $3, $4, $5, 'Locked', NOW(), NOW())
            RETURNING id, wallet_id, project_id, contract_id, amount, currency_code,
                status as "status: EscrowStatus", locked_at, released_at, refunded_at, created_at
            "#,
            request.wallet_id,
            request.project_id,
            request.contract_id,
            request.amount,
            request.currency_code
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create milestone escrow records
        for milestone in request.milestones {
            sqlx::query!(
                "INSERT INTO escrow_milestones (escrow_id, milestone_id, amount, status) VALUES ($1, $2, $3, 'Pending')",
                escrow.id,
                milestone.milestone_id,
                milestone.amount
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
        }

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Send notifications
        self.send_escrow_notification(&escrow, "created").await;

        Ok(escrow)
    }

    /// Release milestone payment
    pub async fn release_milestone(
        &self,
        request: ReleaseMilestoneRequest,
    ) -> Result<(), ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Get escrow and milestone details
        let escrow = sqlx::query_as!(
            WalletEscrow,
            r#"SELECT * FROM wallet_escrows WHERE id = $1 FOR UPDATE"#,
            request.escrow_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Escrow not found".to_string()))?;

        if escrow.status != EscrowStatus::Locked {
            return Err(ApiError::BadRequest("Escrow is not in locked state".to_string()));
        }

        let milestone = sqlx::query_as!(
            EscrowMilestone,
            r#"SELECT * FROM escrow_milestones WHERE escrow_id = $1 AND milestone_id = $2"#,
            request.escrow_id,
            request.milestone_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Milestone not found".to_string()))?;

        // Get freelancer wallet
        let freelancer_wallet_id = sqlx::query_scalar!(
            "SELECT w.id FROM wallets w JOIN contracts c ON c.freelancer_id = w.user_id WHERE c.id = $1",
            escrow.contract_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Freelancer wallet not found".to_string()))?;

        // Release funds
        sqlx::query!(
            "UPDATE wallet_balances SET locked_balance = locked_balance - $1 WHERE wallet_id = $2 AND currency_code = $3",
            milestone.amount,
            escrow.wallet_id,
            escrow.currency_code
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Transfer to freelancer
        sqlx::query!(
            "UPDATE wallet_balances SET balance = balance + $1 WHERE wallet_id = $2 AND currency_code = $3",
            milestone.amount,
            freelancer_wallet_id,
            escrow.currency_code
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Update milestone status
        sqlx::query!(
            "UPDATE escrow_milestones SET status = 'Released', released_at = NOW() WHERE id = $1",
            milestone.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create transaction record
        sqlx::query!(
            r#"
            INSERT INTO wallet_transactions (
                from_wallet_id, to_wallet_id, amount, currency_code, tx_type, status, created_at
            )
            VALUES ($1, $2, $3, $4, 'MilestoneRelease', 'Confirmed', NOW())
            "#,
            escrow.wallet_id,
            freelancer_wallet_id,
            milestone.amount,
            escrow.currency_code
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Send notifications
        self.send_milestone_notification(&escrow, &milestone, "released").await;

        Ok(())
    }

    /// Handle escrow dispute
    pub async fn dispute_escrow(
        &self,
        request: DisputeEscrowRequest,
    ) -> Result<(), ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Update escrow status
        let status = if request.freeze_wallet {
            EscrowStatus::Frozen
        } else {
            EscrowStatus::Disputed
        };

        sqlx::query!(
            "UPDATE wallet_escrows SET status = $1 WHERE id = $2",
            status as EscrowStatus,
            request.escrow_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Create dispute record
        sqlx::query!(
            "INSERT INTO wallet_disputes (escrow_id, reason, status, created_at) VALUES ($1, $2, 'Open', NOW())",
            request.escrow_id,
            request.reason
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Send notifications
        let escrow = self.get_escrow(request.escrow_id).await?;
        self.send_escrow_notification(&escrow, "disputed").await;

        Ok(())
    }

    /// Refund escrow to client
    pub async fn refund_escrow(
        &self,
        request: RefundEscrowRequest,
    ) -> Result<(), ApiError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        let escrow = sqlx::query_as!(
            WalletEscrow,
            "SELECT * FROM wallet_escrows WHERE id = $1 FOR UPDATE",
            request.escrow_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::NotFound("Escrow not found".to_string()))?;

        let refund_amount = request.partial_amount.unwrap_or(escrow.amount);

        // Unlock funds
        sqlx::query!(
            "UPDATE wallet_balances SET balance = balance + $1, locked_balance = locked_balance - $1 WHERE wallet_id = $2 AND currency_code = $3",
            refund_amount,
            escrow.wallet_id,
            escrow.currency_code
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Update escrow status
        sqlx::query!(
            "UPDATE wallet_escrows SET status = 'Refunded', refunded_at = NOW() WHERE id = $1",
            request.escrow_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        // Send notifications
        self.send_escrow_notification(&escrow, "refunded").await;

        Ok(())
    }

    async fn get_escrow(&self, escrow_id: i32) -> Result<WalletEscrow, ApiError> {
        sqlx::query_as!(
            WalletEscrow,
            r#"SELECT * FROM wallet_escrows WHERE id = $1"#,
            escrow_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ApiError::NotFound("Escrow not found".to_string()))
    }

    async fn send_escrow_notification(&self, escrow: &WalletEscrow, action: &str) {
        let message = format!(
            "Escrow {} for project #{}: {} {}",
            action, escrow.project_id, escrow.amount, escrow.currency_code
        );
        
        // Send Telegram notification
        if let Ok(user_id) = sqlx::query_scalar!(
            "SELECT user_id FROM wallets WHERE id = $1",
            escrow.wallet_id
        )
        .fetch_one(&self.pool)
        .await {
            let _ = self.telegram_service.send_escrow_notification(user_id, &message).await;
        }
    }

    async fn send_milestone_notification(&self, escrow: &WalletEscrow, milestone: &EscrowMilestone, action: &str) {
        let message = format!(
            "Milestone #{} {}: {} {}",
            milestone.milestone_id, action, milestone.amount, escrow.currency_code
        );
        
        if let Ok(user_id) = sqlx::query_scalar!(
            "SELECT user_id FROM wallets WHERE id = $1",
            escrow.wallet_id
        )
        .fetch_one(&self.pool)
        .await {
            let _ = self.telegram_service.send_milestone_notification(user_id, &message).await;
        }
    }
}
