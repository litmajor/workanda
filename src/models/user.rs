use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(FromRow, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: i32,
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub user_type: UserType,
    pub role_id: i32,
    pub two_factor_secret: Option<String>,
    pub two_factor_enabled: bool,
    pub privileges: Privileges,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum UserType {
    Client,
    Freelancer,
    Admin,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub profile_picture: Option<String>,
}

pub async fn get_user_by_id(pool: &sqlx::PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Freelancer,
    Client,
    Admin,
}