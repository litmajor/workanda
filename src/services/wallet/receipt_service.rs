use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::services::wallet::wallet_service::WalletError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub transaction_id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub currency: String,
    pub transaction_hash: Option<String>,
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub gas_fee: Option<String>,
}

pub struct ReceiptService {
    pool: PgPool,
}

impl ReceiptService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn generate_receipt(&self, transaction_id: Uuid) -> Result<Receipt, WalletError> {
        let tx = sqlx::query!(
            r#"
            SELECT id, from_address, to_address, amount, currency_code, transaction_hash, 
                   status, gas_fee, created_at
            FROM wallet_transactions
            WHERE id = $1
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch transaction: {}", e)))?;

        match tx {
            None => Err(WalletError::ValidationError("Transaction not found".to_string())),
            Some(transaction) => {
                let receipt = Receipt {
                    transaction_id,
                    from_address: transaction.from_address.unwrap_or_default(),
                    to_address: transaction.to_address.unwrap_or_default(),
                    amount: transaction.amount.to_string(),
                    currency: transaction.currency_code,
                    transaction_hash: transaction.transaction_hash,
                    status: transaction.status,
                    timestamp: transaction.created_at,
                    gas_fee: transaction.gas_fee.map(|f| f.to_string()),
                };

                let receipt_data = json!(receipt);
                
                sqlx::query!(
                    r#"
                    INSERT INTO wallet_receipts (transaction_id, receipt_data)
                    VALUES ($1, $2)
                    ON CONFLICT (transaction_id) DO UPDATE
                    SET receipt_data = $2
                    "#,
                    transaction_id,
                    receipt_data
                )
                .execute(&self.pool)
                .await
                .map_err(|e| WalletError::DatabaseError(format!("Failed to save receipt: {}", e)))?;

                Ok(receipt)
            }
        }
    }

    pub async fn get_receipt(&self, transaction_id: Uuid) -> Result<Option<Receipt>, WalletError> {
        let receipt = sqlx::query!(
            r#"
            SELECT receipt_data
            FROM wallet_receipts
            WHERE transaction_id = $1
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WalletError::DatabaseError(format!("Failed to fetch receipt: {}", e)))?;

        match receipt {
            None => Ok(None),
            Some(r) => {
                let receipt_data: Receipt = serde_json::from_value(r.receipt_data)
                    .map_err(|e| WalletError::CryptoError(format!("Failed to parse receipt: {}", e)))?;
                Ok(Some(receipt_data))
            }
        }
    }

    pub fn generate_receipt_html(&self, receipt: &Receipt) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <title>Transaction Receipt</title>
                <style>
                    body {{ font-family: Arial, sans-serif; max-width: 600px; margin: 40px auto; padding: 20px; }}
                    .header {{ text-align: center; border-bottom: 2px solid #4F46E5; padding-bottom: 20px; }}
                    .details {{ margin: 20px 0; }}
                    .detail-row {{ display: flex; justify-content: space-between; padding: 10px; border-bottom: 1px solid #eee; }}
                    .label {{ font-weight: bold; color: #666; }}
                    .value {{ color: #333; }}
                    .footer {{ margin-top: 40px; text-align: center; color: #888; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="header">
                    <h1>Workanda Wallet Receipt</h1>
                    <p>Transaction Confirmation</p>
                </div>
                <div class="details">
                    <div class="detail-row">
                        <span class="label">Transaction ID:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">From:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">To:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">Amount:</span>
                        <span class="value">{} {}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">Status:</span>
                        <span class="value">{}</span>
                    </div>
                    <div class="detail-row">
                        <span class="label">Timestamp:</span>
                        <span class="value">{}</span>
                    </div>
                    {}
                    {}
                </div>
                <div class="footer">
                    <p>This receipt was generated by Workanda Wallet</p>
                    <p>Powered by Celo Blockchain</p>
                </div>
            </body>
            </html>
            "#,
            receipt.transaction_id,
            receipt.from_address,
            receipt.to_address,
            receipt.amount,
            receipt.currency,
            receipt.status,
            receipt.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            receipt.gas_fee.as_ref().map_or(String::new(), |f| format!(
                r#"<div class="detail-row"><span class="label">Gas Fee:</span><span class="value">{} {}</span></div>"#,
                f, receipt.currency
            )),
            receipt.transaction_hash.as_ref().map_or(String::new(), |h| format!(
                r#"<div class="detail-row"><span class="label">Transaction Hash:</span><span class="value">{}</span></div>"#,
                h
            ))
        )
    }
}
