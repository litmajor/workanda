
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::predictive_analytics::*;
use crate::services::predictive_analytics_service::PredictiveAnalyticsService;
use crate::api::error::ApiError;

pub async fn predict_project_success(
    pool: web::Data<PgPool>,
    request: web::Json<ProjectSuccessPredictionRequest>,
) -> Result<HttpResponse, ApiError> {
    let service = PredictiveAnalyticsService::new(pool.get_ref().clone());

    let assessment = service
        .predict_project_success(request.into_inner())
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(assessment))
}

pub async fn suggest_pricing(
    pool: web::Data<PgPool>,
    request: web::Json<PricingRequest>,
) -> Result<HttpResponse, ApiError> {
    let service = PredictiveAnalyticsService::new(pool.get_ref().clone());

    let suggestion = service
        .suggest_pricing(request.into_inner())
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(suggestion))
}

pub async fn estimate_timeline(
    pool: web::Data<PgPool>,
    request: web::Json<TimelineRequest>,
) -> Result<HttpResponse, ApiError> {
    let service = PredictiveAnalyticsService::new(pool.get_ref().clone());

    let estimation = service
        .estimate_timeline(request.into_inner())
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(estimation))
}
