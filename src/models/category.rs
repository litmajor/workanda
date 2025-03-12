use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String, // e.g., "Programmer", "Writer", "Designer"
}