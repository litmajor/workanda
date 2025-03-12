use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Budget {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewBudget {
    pub user_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct BudgetCategory {
    pub id: i32,
    pub budget_id: i32,
    pub name: String,
    pub planned_amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NewBudgetCategory {
    pub budget_id: i32,
    pub name: String,
    pub planned_amount: f64,
}