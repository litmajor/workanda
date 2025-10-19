
use actix_web::web;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/wallet")
            .route("", web::post().to(handlers::create_wallet))
            .route("", web::get().to(handlers::get_wallets))
            .route("/overview", web::get().to(handlers::get_wallet_overview))
            .route("/deposit", web::post().to(handlers::deposit))
            .route("/withdraw", web::post().to(handlers::withdraw))
            .route("/transfer", web::post().to(handlers::transfer))
            .route("/{wallet_id}/transactions", web::get().to(handlers::get_transaction_history))
    );
}
