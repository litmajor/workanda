use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::contact::ContactInformation;

// Create new contact information
pub async fn create_contact_handler(
    pool: web::Data<PgPool>,
    input: web::Json<ContactInformation>,
) -> Result<HttpResponse, ApiError> {
    let new_contact = sqlx::query_as::<_, ContactInformation>(
        r#"
        INSERT INTO contacts (email, phone)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&input.email)
    .bind(&input.phone)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(new_contact))
}

// Get contact information by ID
pub async fn get_contact_handler(
    pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let contact = sqlx::query_as::<_, ContactInformation>(
        "SELECT * FROM contacts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or(ApiError::NotFound(format!("Contact with ID {} not found", id)))?;

    Ok(HttpResponse::Ok().json(contact))
}