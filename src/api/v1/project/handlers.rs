use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::project::*;
use crate::database::queries;

// Create a new project
pub async fn create_project_handler(
    pool: web::Data<PgPool>,
    project: web::Json<NewProject>,
) -> Result<HttpResponse, ApiError> {
    let created_project = queries::create_project(pool.as_ref(), project.into_inner()).await?;
    Ok(HttpResponse::Created().json(created_project))
}

// Get all projects
pub async fn get_all_projects_handler(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    let projects = queries::get_all_projects(pool.as_ref()).await?;
    Ok(HttpResponse::Ok().json(projects))
}

// Get a specific project by ID
pub async fn get_project_by_id_handler(
    pool: web::Data<PgPool>,
    web::Path(project_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let project = queries::get_project_by_id(pool.as_ref(), project_id)
        .await?
        .ok_or(ApiError::NotFound(format!("Project with ID {} not found", project_id)))?;

    Ok(HttpResponse::Ok().json(project))
}

// Update a project
pub async fn update_project_handler(
    pool: web::Data<PgPool>,
    web::Path(project_id): web::Path<i32>,
    project: web::Json<UpdatedProject>,
) -> Result<HttpResponse, ApiError> {
    let updated_project = queries::update_project(pool.as_ref(), project_id, project.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_project))
}

// Delete a project
pub async fn delete_project_handler(
    pool: web::Data<PgPool>,
    web::Path(project_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = queries::delete_project(pool.as_ref(), project_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!("Project with ID {} not found", project_id)));
    }
    Ok(HttpResponse::NoContent().finish())
}

// Add a freelancer to a project
pub async fn add_freelancer_to_project_handler(
    pool: web::Data<PgPool>,
    web::Path((project_id, freelancer_id)): web::Path<(i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    queries::add_freelancer_to_project(pool.as_ref(), project_id, freelancer_id).await?;
    Ok(HttpResponse::Ok().body("Freelancer added to project"))
}

// Remove a freelancer from a project
pub async fn remove_freelancer_from_project_handler(
    pool: web::Data<PgPool>,
    web::Path((project_id, freelancer_id)): web::Path<(i32, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let rows_affected = queries::remove_freelancer_from_project(pool.as_ref(), project_id, freelancer_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::NotFound(format!(
            "Freelancer with ID {} not found in project {}",
            freelancer_id, project_id
        )));
    }
    Ok(HttpResponse::Ok().body("Freelancer removed from project"))
}