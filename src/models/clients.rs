use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize)]
pub struct ClientAccount {
    pub id: i32,
    pub user_id: Uuid,
    pub company_name: String,
    pub industry: String,
    pub location: String,
    pub contact_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewClientAccount {
    pub user_id: Uuid,
    pub company_name: String,
    pub industry: String,
    pub location: String,
    pub contact_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatedClientAccount {
    pub company_name: Option<String>,
    pub industry: Option<String>,
    pub location: Option<String>,
    pub contact_email: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ClientReview {
    pub id: i32,
    pub client_id: Uuid,
    pub freelancer_id: i32,
    pub feedback: String,
    pub rating: f64,
    pub created_at: chrono::NaiveDateTime,
}