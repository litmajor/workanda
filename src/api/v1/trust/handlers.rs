
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::trust_safety::*;
use crate::services::trust_safety_service::TrustSafetyService;
use crate::api::error::ApiError;

pub async fn get_trust_score(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let service = TrustSafetyService::new(pool.get_ref().clone());
    
    let trust_score = service
        .calculate_trust_score(*user_id)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(trust_score))
}

pub async fn check_fraud(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let service = TrustSafetyService::new(pool.get_ref().clone());
    
    let fraud_result = service
        .detect_fraud(*user_id)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(fraud_result))
}

pub async fn predict_dispute_risk(
    pool: web::Data<PgPool>,
    contract_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let service = TrustSafetyService::new(pool.get_ref().clone());
    
    let risk_alert = service
        .predict_dispute_risk(*contract_id)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(risk_alert))
}

pub async fn get_behavioral_analysis(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let service = TrustSafetyService::new(pool.get_ref().clone());
    
    // This is a private method, but we can expose a public wrapper
    // For now, return trust score which includes behavioral analysis
    let trust_score = service
        .calculate_trust_score(*user_id)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(trust_score))
}

pub async fn get_fraud_alerts(
    pool: web::Data<PgPool>,
    query: web::Query<FraudAlertsQuery>,
) -> Result<HttpResponse, ApiError> {
    let mut sql = "SELECT * FROM fraud_alerts WHERE 1=1".to_string();
    
    if let Some(user_id) = query.user_id {
        sql.push_str(&format!(" AND user_id = '{}'", user_id));
    }
    
    if let Some(status) = &query.status {
        sql.push_str(&format!(" AND status = '{}'", status));
    }
    
    sql.push_str(" ORDER BY created_at DESC LIMIT 100");
    
    let alerts = sqlx::query_as::<_, FraudAlert>(&sql)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;
    
    Ok(HttpResponse::Ok().json(alerts))
}

#[derive(serde::Deserialize)]
pub struct FraudAlertsQuery {
    pub user_id: Option<Uuid>,
    pub status: Option<String>,
}
