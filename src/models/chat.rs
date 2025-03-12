use chrono::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: i32,
    pub name: String,
    pub is_private: bool,
    pub created_at: DateTime<chrono::Utc>,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomMember {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub joined_at: DateTime<chrono::Utc>,
}