use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Interaction {
    pub id: i32,
    pub freelancer_id: String,
    pub project_id: String,
    pub interaction_type: String,
    pub interaction_date: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "interactions"]
pub struct NewInteraction<'a> {
    pub freelancer_id: &'a str,
    pub project_id: &'a str,
    pub interaction_type: &'a str,
    pub interaction_date: NaiveDateTime,
}