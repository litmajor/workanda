use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub budget: Option<f64>,
    pub client_id: Uuid,
    pub freelancer_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub status: ProjectStatus,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ProjectStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub budget: Option<f64>,
    pub client_id: Uuid,
    pub freelancer_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFee {
    pub id: u32,
    pub contract_id: u32,
    pub amount: f32,
    pub fee_type: FeeType,
    pub charged_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeeType {
    Fixed,  // Fixed fee amount
    Percentage(f32), // Percentage of the contract value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStatusHistory {
    pub id: Option<i32>,
    pub project_id: i32,
    pub previous_status: ProjectStatus,
    pub new_status: ProjectStatus,
    pub created_at: DateTime<Utc>,
}



#[derive(Debug, Clone)]
pub enum ProjectVisibility {
    Public,
    Private,
    Shared,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub project_id: u32,
    pub depends_on: u32, // ID of the project this project depends on
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTemplate {
    id: i32,
    name: String,
    default_budget: Option<f64>,
    default_category: Option<String>,
    default_description: Option<String>,
}
