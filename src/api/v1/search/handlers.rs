use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::search::SearchQuery;

pub async fn search_handler(
    pool: web::Data<PgPool>,
    query: web::Json<SearchQuery>,
) -> Result<HttpResponse, ApiError> {
    let results = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT title, description
        FROM projects
        WHERE title ILIKE $1 OR description ILIKE $1
        AND client_id = $2
        "#,
    )
    .bind(format!("%{}%", query.query))
    .bind(query.user_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(results))
}