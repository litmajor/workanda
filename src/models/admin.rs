use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize)]
pub struct AdminDashboard {
    pub total_users: i32,
    pub total_jobs: i32,
    pub total_disputes: i32,
    pub total_revenue: i32,
    pub number_of_clients: i32,
    pub total_diputes: i32,
    pub solved_disputes: i32,
    pub pending_disputes: i32,
}