use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    pub city: String,
    pub state: Option<String>,
    pub country: String,
}