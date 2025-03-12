use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::task_list::TaskList;

// Create a new task list
pub async fn create_task_list_handler(
    pool: web::Data<PgPool>,
    task_list: web::Json<TaskList>,
) -> Result<HttpResponse, ApiError> {
    let new_task_list = sqlx::query_as::<_, TaskList>(
        r#"
        INSERT INTO task_lists (project_id, name)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(task_list.project_id)
    .bind(&task_list.name)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(new_task_list))
}

// Get all task lists for a project
pub async fn get_task_lists_for_project_handler(
    pool: web::Data<PgPool>,
    project_id: web::Path<u32>,
) -> Result<HttpResponse, ApiError> {
    let task_lists = sqlx::query_as::<_, TaskList>(
        "SELECT * FROM task_lists WHERE project_id = $1",
    )
    .bind(*project_id)
    .fetch_all(pool.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(task_lists))
}