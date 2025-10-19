
use actix_web::web;
use super::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/kyc")
            .route("", web::post().to(handlers::submit_kyc))
            .route("/status", web::get().to(handlers::get_kyc_status))
            .route("/limits", web::get().to(handlers::get_limits))
            .route("/pending", web::get().to(handlers::get_pending_verifications))
            .route("/statistics", web::get().to(handlers::get_statistics))
            .route("/{kyc_id}/approve", web::post().to(handlers::approve_kyc))
            .route("/{kyc_id}/reject", web::post().to(handlers::reject_kyc))
    );
}
