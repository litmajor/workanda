use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize)]
pub struct TaskList {
    pub id: u32,
    pub project_id: u32,
    pub name: String,
}