
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContractTemplate {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub content: String,
    pub variables: Vec<String>,
    pub is_public: bool,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub content: String,
    pub variables: Vec<String>,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateContractRequest {
    pub template_id: i32,
    pub variable_values: std::collections::HashMap<String, String>,
    pub client_id: i32,
    pub freelancer_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedContract {
    pub content: String,
    pub contract_id: Option<i32>,
}
