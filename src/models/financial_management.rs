
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct IncomeEntry {
    pub id: i32,
    pub wallet_id: i32,
    pub project_id: Option<i32>,
    pub client_id: Option<i32>,
    pub amount: Decimal,
    pub currency_code: String,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExpenseEntry {
    pub id: i32,
    pub wallet_id: i32,
    pub amount: Decimal,
    pub currency_code: String,
    pub category: ExpenseCategory,
    pub description: String,
    pub receipt_url: Option<String>,
    pub is_business: bool,
    pub date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "expense_category")]
pub enum ExpenseCategory {
    Software,
    Hardware,
    Marketing,
    Travel,
    Office,
    Education,
    Healthcare,
    Entertainment,
    Other,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Budget {
    pub id: i32,
    pub wallet_id: i32,
    pub name: String,
    pub amount: Decimal,
    pub currency_code: String,
    pub period: BudgetPeriod,
    pub alert_threshold: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "budget_period")]
pub enum BudgetPeriod {
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SavingsGoal {
    pub id: i32,
    pub wallet_id: i32,
    pub name: String,
    pub target_amount: Decimal,
    pub current_amount: Decimal,
    pub currency_code: String,
    pub deadline: Option<NaiveDate>,
    pub auto_contribute: bool,
    pub contribution_amount: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: i32,
    pub wallet_id: i32,
    pub invoice_number: String,
    pub client_name: String,
    pub client_email: String,
    pub amount: Decimal,
    pub currency_code: String,
    pub status: InvoiceStatus,
    pub due_date: NaiveDate,
    pub issued_date: NaiveDate,
    pub paid_date: Option<NaiveDate>,
    pub items: serde_json::Value,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "invoice_status")]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxSummary {
    pub total_income: Decimal,
    pub total_expenses: Decimal,
    pub taxable_income: Decimal,
    pub estimated_tax: Decimal,
    pub deductions: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialReport {
    pub period: String,
    pub total_income: Decimal,
    pub total_expenses: Decimal,
    pub net_income: Decimal,
    pub income_by_project: Vec<ProjectIncome>,
    pub expenses_by_category: Vec<CategoryExpense>,
    pub top_clients: Vec<ClientRevenue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIncome {
    pub project_id: i32,
    pub project_name: String,
    pub total: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryExpense {
    pub category: ExpenseCategory,
    pub total: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRevenue {
    pub client_id: i32,
    pub client_name: String,
    pub total: Decimal,
}
