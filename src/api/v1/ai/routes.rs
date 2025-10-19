
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
            .route("/team/suggest", web::post().to(handlers::suggest_team)),
    );
}
