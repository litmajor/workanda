use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct CategoryInput {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CategoryAssignmentInput {
    pub category_id: i32,
}