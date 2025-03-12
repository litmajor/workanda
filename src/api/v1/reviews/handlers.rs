use sqlx::PgPool;
use actix_web::{web, HttpResponse, Responder};
use crate::models::review::ClientReviewInput;

// Add a new client review
pub async fn add_review_handler(
    pool: web::Data<PgPool>,
    input: web::Json<ClientReviewInput>,
) -> Result<HttpResponse, ApiError> {
    let new_review = sqlx::query(
        r#"
        INSERT INTO client_reviews (client_id, client_name, feedback, rating)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(input.client_id)
    .bind(&input.client_name)
    .bind(&input.feedback)
    .bind(input.rating)
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(json!({ "message": "Review added successfully" })))
}



pub async fn get_client_reviews_handler(
    pool: web::Data<PgPool>,
    web::Path(freelancer_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let reviews = sqlx::query_as::<_, ClientReview>(
        "SELECT * FROM client_reviews WHERE freelancer_id = $1",
    )
    .bind(freelancer_id)
    .fetch_all(pool.as_ref())
    .await?;

    info!("Retrieved client reviews for freelancer ID {}", freelancer_id);
    Ok(HttpResponse::Ok().json(reviews))
}

pub async fn add_review_endpoint(
    pool: web::Data<PgPool>,
    review_input: web::Json<(i32, i32, i32, i32, i32, String)>, // client_id, freelancer_id, communication, quality, punctuality, comment
) -> Result<HttpResponse, ApiError> {
    let (client_id, freelancer_id, communication, quality, punctuality, comment) = review_input.into_inner();

    let review = sqlx::query_as::<_, Review>(
        r#"
        INSERT INTO reviews (client_id, freelancer_id, communication_rating, quality_rating, punctuality_rating, comment)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(client_id)
    .bind(freelancer_id)
    .bind(communication)
    .bind(quality)
    .bind(punctuality)
    .bind(&comment)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Review added successfully for freelancer ID {} by client ID {}",
        freelancer_id, client_id
    );
    Ok(HttpResponse::Created().json(review))
}

pub async fn get_reviews_for_freelancer_endpoint(
    pool: web::Data<PgPool>,
    web::Path(freelancer_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let reviews = sqlx::query_as::<_, Review>(
        "SELECT * FROM reviews WHERE freelancer_id = $1",
    )
    .bind(freelancer_id)
    .fetch_all(pool.as_ref())
    .await?;

    info!("Retrieved reviews for freelancer ID {}", freelancer_id);
    Ok(HttpResponse::Ok().json(reviews))
}

pub async fn add_review_response_endpoint(
    pool: web::Data<PgPool>,
    web::Path(review_id): web::Path<i32>,
    response_input: web::Json<String>,
) -> Result<HttpResponse, ApiError> {
    let response = response_input.into_inner();

    let updated_review = sqlx::query_as::<_, Review>(
        r#"
        UPDATE reviews
        SET freelancer_response = $2
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(review_id)
    .bind(&response)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Added response to review ID {}", review_id);
    Ok(HttpResponse::Created().json(updated_review))
}

pub async fn get_responses_for_review_endpoint(
    pool: web::Data<PgPool>,
    web::Path(review_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let responses = sqlx::query_as::<_, ReviewResponse>(
        "SELECT * FROM review_responses WHERE review_id = $1",
    )
    .bind(review_id)
    .fetch_all(pool.as_ref())
    .await?;

    info!("Retrieved responses for review ID {}", review_id);
    Ok(HttpResponse::Ok().json(responses))
}

pub async fn get_aggregate_ratings(
    pool: web::Data<PgPool>,
    web::Path(freelancer_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let ratings = sqlx::query_as::<_, AggregateRatings>(
        r#"
        SELECT 
            AVG(communication_rating) AS communication_rating,
            AVG(quality_rating) AS quality_rating,
            AVG(punctuality_rating) AS punctuality_rating,
            AVG((communication_rating + quality_rating + punctuality_rating) / 3.0) AS overall_rating
        FROM reviews
        WHERE freelancer_id = $1
        "#,
    )
    .bind(freelancer_id)
    .fetch_one(pool.as_ref())
    .await?;

    info!("Retrieved aggregate ratings for freelancer ID {}", freelancer_id);
    Ok(HttpResponse::Ok().json(ratings))
}

pub async fn get_reviews_paginated(
    pool: web::Data<PgPool>,
    web::Path(freelancer_id): web::Path<i32>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let reviews = sqlx::query_as::<_, Review>(
        r#"
        SELECT * FROM reviews
        WHERE freelancer_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(freelancer_id)
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(pool.as_ref())
    .await?;

    let total_reviews: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM reviews WHERE freelancer_id = $1",
    )
    .bind(freelancer_id)
    .fetch_one(pool.as_ref())
    .await?;

    info!(
        "Retrieved paginated reviews for freelancer ID {} (Page: {}, Per Page: {})",
        freelancer_id, page, per_page
    );
    Ok(HttpResponse::Ok().json(PaginatedReviews {
        reviews,
        total_reviews,
        page,
        per_page,
    }))
}

// Update a review
pub async fn update_review_handler(
    pool: web::Data<PgPool>,
    web::Path(review_id): web::Path<i32>,
    input: web::Json<UpdatedReview>,
) -> Result<HttpResponse, ApiError> {
    let updated_review = queries::update_review(pool.as_ref(), review_id, input.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_review))
}

// Delete a review
pub async fn delete_review_handler(
    pool: web::Data<PgPool>,
    web::Path(review_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = queries::delete_review(pool.as_ref(), review_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!("Review with ID {} not found", review_id)));
    }
    Ok(HttpResponse::NoContent().finish())
}
