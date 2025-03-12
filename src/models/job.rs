use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub budget: Option<f64>,
    pub deadline: Option<NaiveDateTime>,
    pub client_id: u32,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewJob {
    pub title: String,
    pub description: Option<String>,
    pub budget: Option<f64>,
    pub deadline: Option<NaiveDateTime>,
    pub client_id: u32,
    pub category: Option<String>,
    pub priority: Option<String>,
}



#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Open,
    Closed,
    Filled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobVisibility {
    Public,
    Private,
}

#[derive(Debug, Clone)]
pub struct JobCategory {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct JobLocation {
    pub id: u32,
    pub city: String,
    pub country: String,
}
