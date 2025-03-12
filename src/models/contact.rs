use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize)]
pub struct ContactInformation {
    pub id: i32,
    pub email: String,
    pub phone: Option<String>,
}