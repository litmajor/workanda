use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, sqlx::FromRow)]
pub struct PaymentHistory {
    pub id: i32, // Primary key for the payment history entry
    pub escrow_account_id: i32, // Foreign key referencing the escrow account
    pub amount: Decimal, // Amount involved in the transaction
    pub transaction_type: String, // Type of transaction (e.g., "Deposit", "Release", "Refund", etc.)
    pub recipient_id: i32, // ID of the recipient (client, freelancer, or platform)
    pub created_at: NaiveDateTime, // Timestamp when the transaction was recorded
}