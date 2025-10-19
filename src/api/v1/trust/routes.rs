
use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/trust")
            .route("/score/{user_id}", web::get().to(handlers::get_trust_score))
            .route("/fraud/check/{user_id}", web::get().to(handlers::check_fraud))
            .route("/fraud/alerts", web::get().to(handlers::get_fraud_alerts))
            .route("/dispute/risk/{contract_id}", web::get().to(handlers::predict_dispute_risk))
            .route("/behavioral/{user_id}", web::get().to(handlers::get_behavioral_analysis)),
    );
}
