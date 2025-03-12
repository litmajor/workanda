use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(FromRow, Serialize, Deserialize)]
pub struct PlannedExpense {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub amount: f64,
    pub due_date: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct NewPlannedExpense {
    pub user_id: i32,
    pub name: String,
    pub amount: f64,
    pub due_date: NaiveDate,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub id: i32,
    pub name: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ExpenseTransaction {
    pub id: i32,
    pub user_id: i32,
    pub category_id: Option<i32>,
    pub amount: f64,
    pub date: NaiveDate,
    pub description: Option<String>,
}