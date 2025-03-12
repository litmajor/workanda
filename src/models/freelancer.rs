use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct FreelancerAccount {
    pub id: i32,
    pub user_id: i32,
    pub hourly_rate: Option<f64>,
    pub project_pricing: Option<f64>,
    pub specializations: Vec<String>,
    pub category: String,
    pub availability: AvailabilityStatus,
    pub location: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewFreelancerAccount {
    pub user_id: i32,
    pub hourly_rate: Option<f64>,
    pub project_pricing: Option<f64>,
    pub specializations: Vec<String>,
    pub category: String,
    pub availability: AvailabilityStatus,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatedFreelancerAccount {
    pub hourly_rate: Option<f64>,
    pub project_pricing: Option<f64>,
    pub specializations: Option<Vec<String>>,
    pub category: Option<String>,
    pub availability: Option<AvailabilityStatus>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AvailabilityStatus {
    Available,
    Unavailable,
}