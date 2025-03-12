use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::moderation::ModerationRequest;

pub async fn moderate_message_handler(
    pool: web::Data<PgPool>,
    request: web::Json<ModerationRequest>,
) -> Result<HttpResponse, ApiError> {
    match &request.action[..] {
        "delete" => {
            sqlx::query(
                r#"
                DELETE FROM messages
                WHERE id = $1
                "#,
            )
            .bind(request.message_id)
            .execute(pool.as_ref())
            .await?;
        }
        "modify" => {
            if let Some(new_content) = &request.new_content {
                sqlx::query(
                    r#"
                    UPDATE messages
                    SET content = $1
                    WHERE id = $2
                    "#,
                )
                .bind(new_content)
                .bind(request.message_id)
                .execute(pool.as_ref())
                .await?;
            } else {
                return Err(ApiError::BadRequest("Missing new content".to_string()));
            }
        }
        _ => return Err(ApiError::BadRequest("Invalid action".to_string())),
    }

    Ok(HttpResponse::Ok().body("Message moderated successfully"))
}