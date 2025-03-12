use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct ContractPayment {
    pub id: u32,
    pub contract_id: u32,
    pub amount: f64,
    pub status: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewContractPayment {
    pub amount: f64,
    pub status: String,
    pub description: Option<String>,
}