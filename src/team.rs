use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub leader_id: Uuid,
    pub skills: Vec<String>,
    pub available: bool,
    pub created_at: chrono::NaiveDateTime,
}