use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::task::TaskDependency;

// Add a task dependency
pub async fn add_dependency_handler(
    pool: web::Data<PgPool>,
    dependency: web::Json<TaskDependency>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query(
        r#"
        INSERT INTO task_dependencies (dependent_task_id, prerequisite_task_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(dependency.dependent_task_id)
    .bind(dependency.prerequisite_task_id)
    .execute(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().body("Dependency added successfully"))
}

// Remove a task dependency
pub async fn remove_dependency_handler(
    pool: web::Data<PgPool>,
    dependency: web::Json<TaskDependency>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = sqlx::query(
        r#"
        DELETE FROM task_dependencies
        WHERE dependent_task_id = $1 AND prerequisite_task_id = $2
        "#,
    )
    .bind(dependency.dependent_task_id)
    .bind(dependency.prerequisite_task_id)
    .execute(pool.as_ref())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Dependency between tasks {} and {} not found",
            dependency.dependent_task_id, dependency.prerequisite_task_id
        )));
    }

    Ok(HttpResponse::Ok().body("Dependency removed successfully"))
}