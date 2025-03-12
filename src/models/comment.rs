use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub task_id: u32,
    pub author_id: Uuid, // Reference to user ID
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}