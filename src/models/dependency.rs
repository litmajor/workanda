use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Dependency {
    pub dependent_task_id: i32,
    pub prerequisite_task_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewDependency {
    pub dependent_task_id: i32,
    pub prerequisite_task_id: i32,
}