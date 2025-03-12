use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::DateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Notification {
    pub id: i32,
    pub project_id: i32,
    pub message: String,
    pub created_at: DateTime,
    pub read: bool,
}