
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SalaryInsight {
    pub category_id: i32,
    pub average_salary: f64,
    pub currency: String,
}