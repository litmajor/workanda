
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::ai_matching::*;
use crate::services::ai_matching_service::AIMatchingService;
use crate::api::error::ApiError;

pub async fn get_matches_for_freelancer(
    pool: web::Data<PgPool>,
    freelancer_id: web::Path<Uuid>,
    query: web::Query<MatchRequest>,
) -> Result<HttpResponse, ApiError> {
    let service = AIMatchingService::new(pool.get_ref().clone());
    let limit = query.limit.unwrap_or(10);

    let matches = service
        .match_freelancer_to_projects(*freelancer_id, limit)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(matches))
}

pub async fn get_matches_for_project(
    pool: web::Data<PgPool>,
    project_id: web::Path<i32>,
    query: web::Query<MatchRequest>,
) -> Result<HttpResponse, ApiError> {
    let service = AIMatchingService::new(pool.get_ref().clone());
    let limit = query.limit.unwrap_or(10);

    let matches = service
        .match_project_to_freelancers(*project_id, limit)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(matches))
}

pub async fn explain_match(
    pool: web::Data<PgPool>,
    request: web::Json<MatchRequest>,
) -> Result<HttpResponse, ApiError> {
    if request.freelancer_id.is_none() || request.project_id.is_none() {
        return Err(ApiError::BadRequest(
            "Both freelancer_id and project_id are required".to_string(),
        ));
    }

    let service = AIMatchingService::new(pool.get_ref().clone());
    let matches = service
        .match_project_to_freelancers(request.project_id.unwrap(), 100)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let match_score = matches
        .iter()
        .find(|m| m.freelancer_id == request.freelancer_id.unwrap())
        .ok_or_else(|| ApiError::NotFound("Match not found".to_string()))?;

    let mut detailed_breakdown = std::collections::HashMap::new();
    detailed_breakdown.insert("skill_match".to_string(), match_score.skill_match);
    detailed_breakdown.insert("experience_match".to_string(), match_score.experience_match);
    detailed_breakdown.insert("budget_fit".to_string(), match_score.budget_fit);
    detailed_breakdown.insert("success_probability".to_string(), match_score.success_probability);

    let strengths: Vec<String> = match_score
        .reasons
        .iter()
        .filter(|r| r.contains("Strong") || r.contains("match"))
        .cloned()
        .collect();

    let potential_concerns: Vec<String> = match_score
        .reasons
        .iter()
        .filter(|r| r.contains("Missing") || r.contains("over budget"))
        .cloned()
        .collect();

    let mut recommendations = Vec::new();
    if match_score.skill_match < 0.7 {
        recommendations.push("Consider additional skills training or partnering with specialists".to_string());
    }
    if match_score.budget_fit < 0.7 {
        recommendations.push("Adjust pricing or scope to better fit budget".to_string());
    }
    if match_score.overall_score > 0.8 {
        recommendations.push("Excellent match! Consider applying immediately".to_string());
    }

    let explanation = MatchExplanation {
        match_score: match_score.clone(),
        detailed_breakdown,
        strengths,
        potential_concerns,
        recommendations,
    };

    Ok(HttpResponse::Ok().json(explanation))
}

pub async fn suggest_team(
    pool: web::Data<PgPool>,
    request: web::Json<TeamCompositionRequest>,
) -> Result<HttpResponse, ApiError> {
    let service = AIMatchingService::new(pool.get_ref().clone());
    let max_team_size = request.max_team_size.unwrap_or(5);

    let suggestion = service
        .suggest_team_composition(request.project_id, max_team_size, request.budget_limit)
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(suggestion))
}
