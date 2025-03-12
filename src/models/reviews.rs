use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    pub client_id: i32,
    pub freelancer_id: i32,
    pub communication_rating: i32,
    pub quality_rating: i32,
    pub punctuality_rating: i32,
    pub comment: String,
    pub freelancer_response: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub id: i32,
    pub review_id: i32,
    pub response: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct AggregateRatings {
    pub communication_rating: f64,
    pub quality_rating: f64,
    pub punctuality_rating: f64,
    pub overall_rating: f64,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Serialize)]
pub struct PaginatedReviews {
    pub reviews: Vec<Review>,
    pub total_reviews: i64,
    pub page: i32,
    pub per_page: i32,
}


#[derive(Serialize, Deserialize)]
pub struct ClientReview {
    pub client_id: i32,
    pub client_name: String,
    pub feedback: String,
    pub rating: f64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatedReview {
    pub communication_rating: Option<i32>,
    pub quality_rating: Option<i32>,
    pub punctuality_rating: Option<i32>,
    pub feedback: Option<String>,
}
