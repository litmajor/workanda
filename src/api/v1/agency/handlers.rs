
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::agency::Agency;
use crate::api::error::ApiError;

// Create a new agency
pub async fn create_agency_handler(
    pool: web::Data<PgPool>,
    agency: web::Json<Agency>,
) -> Result<HttpResponse, ApiError> {
    let agency_id = Uuid::new_v4();
    
    let created_agency = sqlx::query_as::<_, Agency>(
        r#"
        INSERT INTO agencies (id, name, owner_id, team_ids, verified, reputation_score, categories, projects_completed, avg_delivery_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#
    )
    .bind(agency_id)
    .bind(&agency.name)
    .bind(agency.owner_id)
    .bind(&agency.team_ids)
    .bind(false)
    .bind(0.0)
    .bind(&agency.categories)
    .bind(0)
    .bind(0)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Created().json(created_agency))
}

// Get agency by ID
pub async fn get_agency_handler(
    pool: web::Data<PgPool>,
    agency_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let agency = sqlx::query_as::<_, Agency>("SELECT * FROM agencies WHERE id = $1")
        .bind(*agency_id)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .ok_or(ApiError::NotFound(format!("Agency {} not found", agency_id)))?;

    Ok(HttpResponse::Ok().json(agency))
}

// Update agency
pub async fn update_agency_handler(
    pool: web::Data<PgPool>,
    agency_id: web::Path<Uuid>,
    agency: web::Json<Agency>,
) -> Result<HttpResponse, ApiError> {
    let updated_agency = sqlx::query_as::<_, Agency>(
        r#"
        UPDATE agencies
        SET name = $2, categories = $3, team_ids = $4
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(*agency_id)
    .bind(&agency.name)
    .bind(&agency.categories)
    .bind(&agency.team_ids)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(updated_agency))
}

// Add team to agency
pub async fn add_team_to_agency_handler(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (agency_id, team_id) = path.into_inner();
    
    sqlx::query(
        r#"
        UPDATE agencies
        SET team_ids = array_append(team_ids, $2)
        WHERE id = $1 AND NOT ($2 = ANY(team_ids))
        "#
    )
    .bind(agency_id)
    .bind(team_id)
    .execute(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Team added to agency successfully"
    })))
}

// Remove team from agency
pub async fn remove_team_from_agency_handler(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiError> {
    let (agency_id, team_id) = path.into_inner();
    
    sqlx::query(
        r#"
        UPDATE agencies
        SET team_ids = array_remove(team_ids, $2)
        WHERE id = $1
        "#
    )
    .bind(agency_id)
    .bind(team_id)
    .execute(pool.as_ref())
    .await
    .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Team removed from agency successfully"
    })))
}

// Get all teams in an agency
pub async fn get_agency_teams_handler(
    pool: web::Data<PgPool>,
    agency_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let agency = sqlx::query_as::<_, Agency>("SELECT * FROM agencies WHERE id = $1")
        .bind(*agency_id)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?
        .ok_or(ApiError::NotFound(format!("Agency {} not found", agency_id)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "agency_id": agency_id,
        "team_ids": agency.team_ids
    })))
}
