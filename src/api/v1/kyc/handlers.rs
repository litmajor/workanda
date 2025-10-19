
use actix_web::{web, HttpResponse};
use crate::services::kyc_service::KycService;
use crate::models::kyc::SubmitKycRequest;
use crate::api::error::ApiError;
use crate::middleware::auth::AuthenticatedUser;
use uuid::Uuid;

pub async fn submit_kyc(
    user: AuthenticatedUser,
    service: web::Data<KycService>,
    request: web::Json<SubmitKycRequest>,
) -> Result<HttpResponse, ApiError> {
    let kyc = service.submit_kyc(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(kyc))
}

pub async fn get_kyc_status(
    user: AuthenticatedUser,
    service: web::Data<KycService>,
) -> Result<HttpResponse, ApiError> {
    let kyc = service.get_kyc_status(user.user_id).await?;
    Ok(HttpResponse::Ok().json(kyc))
}

pub async fn get_limits(
    user: AuthenticatedUser,
    service: web::Data<KycService>,
) -> Result<HttpResponse, ApiError> {
    let limits = service.get_limits(user.user_id).await?;
    Ok(HttpResponse::Ok().json(limits))
}

pub async fn approve_kyc(
    user: AuthenticatedUser,
    service: web::Data<KycService>,
    path: web::Path<Uuid>,
    body: web::Json<ApproveKycRequest>,
) -> Result<HttpResponse, ApiError> {
    // TODO: Check if user is admin
    let kyc = service.approve_kyc(
        path.into_inner(),
        body.provider.clone(),
        body.provider_verification_id.clone()
    ).await?;
    Ok(HttpResponse::Ok().json(kyc))
}

pub async fn reject_kyc(
    user: AuthenticatedUser,
    service: web::Data<KycService>,
    path: web::Path<Uuid>,
    body: web::Json<RejectKycRequest>,
) -> Result<HttpResponse, ApiError> {
    // TODO: Check if user is admin
    let kyc = service.reject_kyc(path.into_inner(), body.reason.clone()).await?;
    Ok(HttpResponse::Ok().json(kyc))
}

pub async fn get_pending_verifications(
    user: AuthenticatedUser,
    service: web::Data<KycService>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, ApiError> {
    // TODO: Check if user is admin
    let verifications = service.get_pending_verifications(query.limit, query.offset).await?;
    Ok(HttpResponse::Ok().json(verifications))
}

pub async fn get_statistics(
    user: AuthenticatedUser,
    service: web::Data<KycService>,
) -> Result<HttpResponse, ApiError> {
    // TODO: Check if user is admin
    let stats = service.get_statistics().await?;
    Ok(HttpResponse::Ok().json(stats))
}

#[derive(serde::Deserialize)]
pub struct ApproveKycRequest {
    pub provider: Option<String>,
    pub provider_verification_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct RejectKycRequest {
    pub reason: String,
}

#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}
