use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde_json::json;

// Update availability status for a user
pub async fn update_availability_handler(
    pool: web::Data<PgPool>,
    web::Path(user_id): web::Path<i32>,
    input: web::Json<AvailabilityStatus>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE users
        SET availability_status = $1
        WHERE id = $2
        "#,
    )
    .bind(input.into_inner().to_string())
    .bind(user_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!("User with ID {} not found", user_id)));
    }

    Ok(HttpResponse::Ok().json(json!({ "message": "Availability updated successfully" })))
}