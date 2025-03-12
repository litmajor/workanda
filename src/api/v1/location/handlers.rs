use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::location::Location;

// Create a new location
pub async fn create_location_handler(
    pool: web::Data<PgPool>,
    input: web::Json<Location>,
) -> Result<HttpResponse, ApiError> {
    let new_location = sqlx::query_as::<_, Location>(
        r#"
        INSERT INTO locations (city, state, country)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&input.city)
    .bind(&input.state)
    .bind(&input.country)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(new_location))
}

// Get all locations
pub async fn get_all_locations_handler(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let locations = sqlx::query_as::<_, Location>("SELECT * FROM locations")
        .fetch_all(pool.as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(locations))
}