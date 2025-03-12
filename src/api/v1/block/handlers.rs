use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::block::BlockedUser;

pub async fn block_user_handler(
    pool: web::Data<PgPool>,
    block_request: web::Json<BlockRequest>,
    blocker_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query(
        r#"
        INSERT INTO blocked_users (blocker_id, blocked_id, created_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(*blocker_id)
    .bind(block_request.blocked_user_id)
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().body("User blocked successfully"))
}

pub async fn unblock_user_handler(
    pool: web::Data<PgPool>,
    block_request: web::Json<BlockRequest>,
    blocker_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query(
        r#"
        DELETE FROM blocked_users
        WHERE blocker_id = $1 AND blocked_id = $2
        "#,
    )
    .bind(*blocker_id)
    .bind(block_request.blocked_user_id)
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().body("User unblocked successfully"))
}