
use sqlx::PgPool;
use rust_decimal::Decimal;
use chrono::{Utc, NaiveDate, Datelike};
use crate::models::financial_management::*;
use crate::api::error::ApiError;

pub struct FinancialManagementService {
    pool: PgPool,
}

impl FinancialManagementService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Track income
    pub async fn track_income(
        &self,
        wallet_id: i32,
        project_id: Option<i32>,
        client_id: Option<i32>,
        amount: Decimal,
        currency_code: String,
        category: String,
        description: String,
        date: NaiveDate,
    ) -> Result<IncomeEntry, ApiError> {
        let income = sqlx::query_as!(
            IncomeEntry,
            r#"
            INSERT INTO income_entries (
                wallet_id, project_id, client_id, amount, currency_code, category, description, date, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            RETURNING *
            "#,
            wallet_id, project_id, client_id, amount, currency_code, category, description, date
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(income)
    }

    /// Track expense
    pub async fn track_expense(
        &self,
        wallet_id: i32,
        amount: Decimal,
        currency_code: String,
        category: ExpenseCategory,
        description: String,
        receipt_url: Option<String>,
        is_business: bool,
        date: NaiveDate,
    ) -> Result<ExpenseEntry, ApiError> {
        let expense = sqlx::query_as!(
            ExpenseEntry,
            r#"
            INSERT INTO expense_entries (
                wallet_id, amount, currency_code, category, description, receipt_url, is_business, date, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            RETURNING id, wallet_id, amount, currency_code, category as "category: ExpenseCategory",
                description, receipt_url, is_business, date, created_at
            "#,
            wallet_id, amount, currency_code, category as ExpenseCategory, description, receipt_url, is_business, date
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(expense)
    }

    /// Create budget
    pub async fn create_budget(
        &self,
        wallet_id: i32,
        name: String,
        amount: Decimal,
        currency_code: String,
        period: BudgetPeriod,
        alert_threshold: Decimal,
    ) -> Result<Budget, ApiError> {
        let budget = sqlx::query_as!(
            Budget,
            r#"
            INSERT INTO budgets (wallet_id, name, amount, currency_code, period, alert_threshold, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id, wallet_id, name, amount, currency_code, period as "period: BudgetPeriod",
                alert_threshold, created_at
            "#,
            wallet_id, name, amount, currency_code, period as BudgetPeriod, alert_threshold
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(budget)
    }

    /// Create savings goal
    pub async fn create_savings_goal(
        &self,
        wallet_id: i32,
        name: String,
        target_amount: Decimal,
        currency_code: String,
        deadline: Option<NaiveDate>,
        auto_contribute: bool,
        contribution_amount: Option<Decimal>,
    ) -> Result<SavingsGoal, ApiError> {
        let goal = sqlx::query_as!(
            SavingsGoal,
            r#"
            INSERT INTO savings_goals (
                wallet_id, name, target_amount, current_amount, currency_code, deadline,
                auto_contribute, contribution_amount, created_at
            )
            VALUES ($1, $2, $3, 0, $4, $5, $6, $7, NOW())
            RETURNING *
            "#,
            wallet_id, name, target_amount, currency_code, deadline, auto_contribute, contribution_amount
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(goal)
    }

    /// Generate invoice
    pub async fn generate_invoice(
        &self,
        wallet_id: i32,
        client_name: String,
        client_email: String,
        amount: Decimal,
        currency_code: String,
        due_date: NaiveDate,
        items: serde_json::Value,
        notes: Option<String>,
    ) -> Result<Invoice, ApiError> {
        let invoice_number = format!("INV-{}-{}", Utc::now().timestamp(), wallet_id);
        
        let invoice = sqlx::query_as!(
            Invoice,
            r#"
            INSERT INTO invoices (
                wallet_id, invoice_number, client_name, client_email, amount, currency_code,
                status, due_date, issued_date, items, notes, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, 'Sent', $7, $8, $9, $10, NOW())
            RETURNING id, wallet_id, invoice_number, client_name, client_email, amount, currency_code,
                status as "status: InvoiceStatus", due_date, issued_date, paid_date, items, notes, created_at
            "#,
            wallet_id, invoice_number, client_name, client_email, amount, currency_code,
            due_date, Utc::now().date_naive(), items, notes
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(invoice)
    }

    /// Calculate tax summary
    pub async fn calculate_tax_summary(
        &self,
        wallet_id: i32,
        year: i32,
    ) -> Result<TaxSummary, ApiError> {
        let total_income = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(amount), 0) FROM income_entries WHERE wallet_id = $1 AND EXTRACT(YEAR FROM date) = $2",
            wallet_id, year as f64
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(Decimal::ZERO);

        let total_expenses = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(amount), 0) FROM expense_entries WHERE wallet_id = $1 AND is_business = true AND EXTRACT(YEAR FROM date) = $2",
            wallet_id, year as f64
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .unwrap_or(Decimal::ZERO);

        let taxable_income = total_income - total_expenses;
        let estimated_tax = taxable_income * Decimal::from_str_exact("0.25").unwrap(); // 25% tax rate

        Ok(TaxSummary {
            total_income,
            total_expenses,
            taxable_income,
            estimated_tax,
            deductions: total_expenses,
        })
    }

    /// Generate financial report
    pub async fn generate_financial_report(
        &self,
        wallet_id: i32,
        period: String,
    ) -> Result<FinancialReport, ApiError> {
        let (start_date, end_date) = self.parse_period(&period)?;

        let total_income = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(amount), 0) FROM income_entries WHERE wallet_id = $1 AND date >= $2 AND date <= $3",
            wallet_id, start_date, end_date
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(Decimal::ZERO);

        let total_expenses = sqlx::query_scalar!(
            "SELECT COALESCE(SUM(amount), 0) FROM expense_entries WHERE wallet_id = $1 AND date >= $2 AND date <= $3",
            wallet_id, start_date, end_date
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(Decimal::ZERO);

        Ok(FinancialReport {
            period: period.clone(),
            total_income,
            total_expenses,
            net_income: total_income - total_expenses,
            income_by_project: vec![],
            expenses_by_category: vec![],
            top_clients: vec![],
        })
    }

    fn parse_period(&self, period: &str) -> Result<(NaiveDate, NaiveDate), ApiError> {
        let now = Utc::now().date_naive();
        match period {
            "monthly" => {
                let start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
                    .ok_or_else(|| ApiError::BadRequest("Invalid date".to_string()))?;
                let end = now;
                Ok((start, end))
            }
            "quarterly" => {
                let quarter_month = ((now.month() - 1) / 3) * 3 + 1;
                let start = NaiveDate::from_ymd_opt(now.year(), quarter_month, 1)
                    .ok_or_else(|| ApiError::BadRequest("Invalid date".to_string()))?;
                Ok((start, now))
            }
            "yearly" => {
                let start = NaiveDate::from_ymd_opt(now.year(), 1, 1)
                    .ok_or_else(|| ApiError::BadRequest("Invalid date".to_string()))?;
                Ok((start, now))
            }
            _ => Err(ApiError::BadRequest("Invalid period".to_string())),
        }
    }
}
