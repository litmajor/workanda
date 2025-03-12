use chrono::DateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockedUser {
    pub id: i32,
    pub blocker_id: i32,
    pub blocked_id: i32,
    pub created_at: DateTime<chrono::Utc>,
}