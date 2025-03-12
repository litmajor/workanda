// Shared Protobuf models (for gRPC)
tonic::include_proto!("workanda.models");

// Shared database models
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// Unique identifier for the user.
    pub id: Uuid,

    /// Email address of the user.
    pub email: String,

    /// Hashed password of the user.
    pub password_hash: String,

    /// Role of the user (e.g., Admin, User).
    pub role: UserRole,

    /// Timestamp when the user was created.
    pub created_at: DateTime<Utc>,
}