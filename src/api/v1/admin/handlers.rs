use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn get_all_users(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool.as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn approve_job(
    pool: web::Data<PgPool>,
    job_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE jobs
        SET status = 'approved'
        WHERE id = $1 AND status = 'pending'
        "#,
    )
    .bind(*job_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Job with ID {} not found or already approved",
            job_id
        )));
    }

    Ok(HttpResponse::Ok().body("Job approved successfully"))
}

pub async fn escalate_dispute(
    pool: web::Data<PgPool>,
    dispute_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE disputes
        SET status = 'escalated'
        WHERE id = $1 AND status = 'open'
        "#,
    )
    .bind(*dispute_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Dispute with ID {} not found or already escalated",
            dispute_id
        )));
    }

    Ok(HttpResponse::Ok().body("Dispute escalated successfully"))
}

pub async fn resolve_dispute(
    pool: web::Data<PgPool>,
    dispute_id: web::Path<i32>,
    resolution: web::Json<String>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        UPDATE disputes
        SET status = 'resolved', resolution = $2
        WHERE id = $1 AND status = 'escalated'
        "#,
    )
    .bind(*dispute_id)
    .bind(resolution.into_inner())
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Dispute with ID {} not found or cannot be resolved",
            dispute_id
        )));
    }

    Ok(HttpResponse::Ok().body("Dispute resolved successfully"))
}