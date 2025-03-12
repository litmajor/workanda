use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::file::NewFileEntry;

pub async fn upload_file_handler(
    pool: web::Data<PgPool>,
    file_entry: web::Json<NewFileEntry>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query(
        r#"
        INSERT INTO file_entries (filename, url)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&file_entry.filename)
    .bind(&file_entry.url)
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().body("File uploaded successfully"))
}