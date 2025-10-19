use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::services::wallet::{
    WalletService, BalanceService, TransactionService, QrService,
    CreateWalletRequest, RecoverWalletRequest, ImportWalletRequest,
    CreateTransactionRequest, TransactionFilter,
};

#[derive(Deserialize)]
pub struct UserIdPath {
    user_id: Uuid,
}

#[derive(Deserialize)]
pub struct WalletIdPath {
    wallet_id: Uuid,
}

#[derive(Deserialize)]
pub struct QrRequest {
    address: String,
    currency: String,
    amount: Option<f64>,
}

pub async fn create_wallet(
    pool: web::Data<PgPool>,
    request: web::Json<CreateWalletRequest>,
) -> impl Responder {
    let wallet_service = WalletService::new(pool.get_ref().clone());
    
    match wallet_service.create_wallet(request.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn recover_wallet(
    pool: web::Data<PgPool>,
    request: web::Json<RecoverWalletRequest>,
) -> impl Responder {
    let wallet_service = WalletService::new(pool.get_ref().clone());
    
    match wallet_service.recover_wallet(request.into_inner()).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn import_wallet(
    pool: web::Data<PgPool>,
    request: web::Json<ImportWalletRequest>,
) -> impl Responder {
    let wallet_service = WalletService::new(pool.get_ref().clone());
    
    match wallet_service.import_wallet(request.into_inner()).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn get_user_wallet(
    pool: web::Data<PgPool>,
    path: web::Path<UserIdPath>,
) -> impl Responder {
    let wallet_service = WalletService::new(pool.get_ref().clone());
    
    match wallet_service.get_wallet_by_user(path.user_id).await {
        Ok(Some(wallet)) => HttpResponse::Ok().json(wallet),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Wallet not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn get_wallet_balances(
    pool: web::Data<PgPool>,
    path: web::Path<WalletIdPath>,
) -> impl Responder {
    let balance_service = BalanceService::new(pool.get_ref().clone());
    
    match balance_service.get_wallet_balances(path.wallet_id).await {
        Ok(balances) => HttpResponse::Ok().json(balances),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn get_portfolio_summary(
    pool: web::Data<PgPool>,
    path: web::Path<WalletIdPath>,
) -> impl Responder {
    let balance_service = BalanceService::new(pool.get_ref().clone());
    
    match balance_service.get_total_portfolio_value(path.wallet_id).await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn create_transaction(
    pool: web::Data<PgPool>,
    request: web::Json<CreateTransactionRequest>,
) -> impl Responder {
    let transaction_service = TransactionService::new(pool.get_ref().clone());
    let wallet_service = WalletService::new(pool.get_ref().clone());
    
    let from_address = match sqlx::query_scalar!(
        "SELECT celo_address FROM wallets WHERE id = $1",
        request.wallet_id
    )
    .fetch_one(pool.get_ref())
    .await {
        Ok(addr) => addr,
        Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Wallet not found: {}", e)
        })),
    };
    
    match transaction_service.create_transaction(request.into_inner(), &from_address).await {
        Ok(transaction) => HttpResponse::Created().json(transaction),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn get_transaction_history(
    pool: web::Data<PgPool>,
    path: web::Path<WalletIdPath>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let transaction_service = TransactionService::new(pool.get_ref().clone());
    let limit = query.get("limit")
        .and_then(|l| l.parse::<i64>().ok())
        .unwrap_or(50);
    
    match transaction_service.get_transaction_history(path.wallet_id, limit).await {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub async fn generate_payment_qr(
    request: web::Json<QrRequest>,
) -> impl Responder {
    let qr_service = QrService::new();
    
    match qr_service.generate_payment_qr(&request.address, &request.currency, request.amount) {
        Ok(qr_code) => HttpResponse::Ok().json(serde_json::json!({
            "qr_code": qr_code
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/wallet")
            .route("/create", web::post().to(create_wallet))
            .route("/recover", web::post().to(recover_wallet))
            .route("/import", web::post().to(import_wallet))
            .route("/user/{user_id}", web::get().to(get_user_wallet))
            .route("/{wallet_id}/balances", web::get().to(get_wallet_balances))
            .route("/{wallet_id}/portfolio", web::get().to(get_portfolio_summary))
            .route("/transaction/create", web::post().to(create_transaction))
            .route("/{wallet_id}/transactions", web::get().to(get_transaction_history))
            .route("/qr/generate", web::post().to(generate_payment_qr))
    );
}
