
use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ai")
            .route(
                "/matches/freelancer/{id}",
                web::get().to(handlers::get_matches_for_freelancer),
            )
            .route(
                "/matches/project/{id}",
                web::get().to(handlers::get_matches_for_project),
            )
            .route("/matches/explain", web::post().to(handlers::explain_match))
            .route("/team/suggest", web::post().to(handlers::suggest_team))
            .route("/team/dynamic", web::post().to(handlers::form_dynamic_team))
            .route("/team/synergy", web::post().to(handlers::analyze_skill_synergy))
            .route("/proposal/analyze", web::post().to(handlers::analyze_proposal))
            .route("/job/categorize/{id}", web::get().to(handlers::categorize_job))
            .route("/search", web::post().to(handlers::smart_search)),
    );
}
