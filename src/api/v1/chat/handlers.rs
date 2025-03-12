use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::chat::{ChatRoom, RoomMember};

pub async fn create_chat_room_handler(
    pool: web::Data<PgPool>,
    room: web::Json<ChatRoom>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query_as::<_, ChatRoom>(
        r#"
        INSERT INTO chat_rooms (name, is_private, created_at, created_by)
        VALUES ($1, $2, NOW(), $3)
        RETURNING *
        "#,
    )
    .bind(&room.name)
    .bind(room.is_private)
    .bind(room.created_by)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().body("Chat room created successfully"))
}

pub async fn join_chat_room_handler(
    pool: web::Data<PgPool>,
    member: web::Json<RoomMember>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query(
        r#"
        INSERT INTO room_members (room_id, user_id, joined_at)
        VALUES ($1, $2, NOW())
        "#,
    )
    .bind(member.room_id)
    .bind(member.user_id)
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().body("User joined chat room successfully"))
}