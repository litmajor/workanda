use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::DateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Contract {
    pub id: i32,
    pub client_id: i32,
    pub freelancer_id: i32,
    pub title: String,
    pub description: String,
    pub value: f32,
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub status: ContractStatus,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ContractStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
pub struct NewContract {
    pub client_id: i32,
    pub freelancer_id: i32,
    pub title: String,
    pub description: String,
    pub value: f32,
    pub start_date: DateTime,
    pub end_date: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractStatus {
    Pending, // After proposal acceptance but before milestones are completed
    Active,
    Completed,
    Cancelled,
}

impl Contract {
    // Optionally, add methods to the Contract struct
    pub fn is_active(&self) -> bool {
        self.status == ContractStatus::Active
    }
}

