
use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/predictive")
            .route("/success", web::post().to(handlers::predict_project_success))
            .route("/pricing", web::post().to(handlers::suggest_pricing))
            .route("/timeline", web::post().to(handlers::estimate_timeline)),
    );
}
