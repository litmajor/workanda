use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::DateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Milestone {
    pub id: i32,
    pub project_id: i32,
    pub description: String,
    pub due_date: DateTime,
    pub completion_status: bool,
    pub associated_payment: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NewMilestone {
    pub project_id: i32,
    pub description: String,
    pub due_date: DateTime,
    pub associated_payment: f64,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ContractMilestone {
    pub id: u32,
    pub contract_id: u32,
    pub title: String,
    pub description: String,
    pub due_date: Option<NaiveDateTime>,
    pub status: String, // e.g., "in_progress", "completed"
    pub payment_amount: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "milestone_progress_reports"]
pub struct MilestoneProgressReport {
    pub id: i32,
    pub milestone_id: i32,
    pub freelancer_id: i32,
    pub report_text: String,
    pub submission_date: NaiveDateTime,
    pub status: String,
    pub attachment_url: Option<String>,
}