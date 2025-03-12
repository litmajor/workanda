use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{category::Category, freelancer_category::FreelancerCategory};
use crate::error::ApiError;

// Create a new category
pub async fn create_category_handler(
    pool: web::Data<PgPool>,
    input: web::Json<CategoryInput>,
) -> Result<HttpResponse, ApiError> {
    let new_category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (name)
        VALUES ($1)
        RETURNING *
        "#,
    )
    .bind(&input.name)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(new_category))
}

// Get all categories
pub async fn get_categories_handler(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(pool.as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(categories))
}

// Assign a category to a freelancer
pub async fn assign_category_handler(
    pool: web::Data<PgPool>,
    input: web::Json<CategoryAssignmentInput>,
    freelancer_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        INSERT INTO freelancer_categories (freelancer_id, category_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(*freelancer_id)
    .bind(input.category_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::Conflict(format!(
            "Category {} is already assigned to freelancer {}",
            input.category_id, *freelancer_id
        )));
    }

    Ok(HttpResponse::Ok().body("Category assigned successfully"))
}

// Remove a category from a freelancer
pub async fn remove_category_handler(
    pool: web::Data<PgPool>,
    input: web::Json<CategoryAssignmentInput>,
    freelancer_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        DELETE FROM freelancer_categories
        WHERE freelancer_id = $1 AND category_id = $2
        "#,
    )
    .bind(*freelancer_id)
    .bind(input.category_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Category {} not found for freelancer {}",
            input.category_id, *freelancer_id
        )));
    }

    Ok(HttpResponse::Ok().body("Category removed successfully"))
}

// Get categories for a specific freelancer
pub async fn get_freelancer_categories_handler(
    pool: web::Data<PgPool>,
    freelancer_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let categories = sqlx::query_as::<_, Category>(
        r#"
        SELECT c.*
        FROM categories c
        INNER JOIN freelancer_categories fc ON c.id = fc.category_id
        WHERE fc.freelancer_id = $1
        "#,
    )
    .bind(*freelancer_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(categories))
}

// Get salary insights for a specific category
pub async fn get_salary_insight_handler(
    pool: web::Data<PgPool>,
    category_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let insight = sqlx::query_as::<_, SalaryInsight>(
        r#"
        SELECT 
            c.id AS category_id,
            AVG(fc.hourly_rate) AS average_salary,
            'USD' AS currency -- Assuming USD for simplicity; you can extend this
        FROM categories c
        LEFT JOIN freelancer_categories fc ON c.id = fc.category_id
        WHERE c.id = $1
        GROUP BY c.id
        "#,
    )
    .bind(*category_id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or(ApiError::NotFound(format!("Category with ID {} not found", category_id)))?;

    Ok(HttpResponse::Ok().json(insight))
}