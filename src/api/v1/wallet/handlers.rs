
use actix_web::{web, HttpResponse};
use crate::services::wallet_service::WalletService;
use crate::models::wallet::*;
use crate::api::error::ApiError;
use crate::middleware::auth::AuthenticatedUser;

pub async fn create_wallet(
    user: AuthenticatedUser,
    service: web::Data<WalletService>,
    request: web::Json<CreateWalletRequest>,
) -> Result<HttpResponse, ApiError> {
    let wallet = service.create_wallet(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(wallet))
}

pub async fn get_wallets(
    user: AuthenticatedUser,
    service: web::Data<WalletService>,
) -> Result<HttpResponse, ApiError> {
    let wallets = service.get_user_wallets(user.user_id).await?;
    Ok(HttpResponse::Ok().json(wallets))
}

pub async fn deposit(
    user: AuthenticatedUser,
    service: web::Data<WalletService>,
    request: web::Json<DepositRequest>,
) -> Result<HttpResponse, ApiError> {
    let transaction = service.deposit(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(transaction))
}

pub async fn withdraw(
    user: AuthenticatedUser,
    service: web::Data<WalletService>,
    request: web::Json<WithdrawalRequest>,
) -> Result<HttpResponse, ApiError> {
    let transaction = service.withdraw(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(transaction))
}

pub async fn transfer(
    user: AuthenticatedUser,
    service: web::Data<WalletService>,
    request: web::Json<TransferRequest>,
) -> Result<HttpResponse, ApiError> {
    let (from_tx, to_tx) = service.transfer(user.user_id, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "from_transaction": from_tx,
        "to_transaction": to_tx
    })))
}

pub async fn get_wallet_overview(
    user: AuthenticatedUser,
    service: web::Data<WalletService>,
) -> Result<HttpResponse, ApiError> {
    let overview = service.get_wallet_overview(user.user_id).await?;
    Ok(HttpResponse::Ok().json(overview))
}

pub async fn get_transaction_history(
    user: AuthenticatedUser,
    service: web::Data<WalletService>,
    path: web::Path<i32>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, ApiError> {
    let wallet_id = path.into_inner();
    let transactions = service.get_transaction_history(
        wallet_id,
        user.user_id,
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await?;
    Ok(HttpResponse::Ok().json(transactions))
}

#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
