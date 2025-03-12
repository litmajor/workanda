use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize)]
pub struct FreelancerCategory {
    pub freelancer_id: i32,
    pub category_id: i32,
}