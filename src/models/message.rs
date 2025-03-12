use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
    pub timestamp: NaiveDateTime,
    pub delivery_status: String,
    pub read_status: String,
    pub is_flagged: bool,
    pub moderation_note: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub content: String,
}