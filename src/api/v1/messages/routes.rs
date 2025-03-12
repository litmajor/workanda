use actix_web::{web, HttpResponse, Responder};
use crate::database::DbPool;
use crate::models::message::{Message, NewMessage};

pub async fn send_message_handler(
    pool: web::Data<DbPool>,
    data: web::Json<NewMessage>,
) -> impl Responder {
    match crate::database::queries::send_message(pool.as_ref(), data.into_inner()).await {
        Ok(message) => HttpResponse::Created().json(message),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_messages_handler(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match crate::database::queries::get_messages_for_user(pool.as_ref(), *user_id).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}