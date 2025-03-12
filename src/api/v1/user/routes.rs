use actix_web::{web, HttpResponse, Responder};
use crate::database::DbPool;
use crate::models::user::{User, NewUser};

pub async fn create_user_handler(
    pool: web::Data<DbPool>,
    data: web::Json<NewUser>,
) -> impl Responder {
    match crate::database::queries::create_user(pool.as_ref(), data.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_handler(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match crate::database::queries::get_user_by_id(pool.as_ref(), *user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}