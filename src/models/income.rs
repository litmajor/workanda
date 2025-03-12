use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(FromRow, Serialize, Deserialize)]
pub struct IncomeSource {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewIncomeSource {
    pub user_id: i32,
    pub name: String,
    pub amount: f64,
    pub date: NaiveDate,
    pub details: Option<String>,
}