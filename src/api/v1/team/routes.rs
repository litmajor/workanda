
use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teams")
            .route("", web::post().to(handlers::create_team_handler))
            .route("/{team_id}", web::get().to(handlers::get_team_handler))
            .route("/{team_id}", web::put().to(handlers::update_team_handler))
            .route("/proposals/submit", web::post().to(handlers::submit_team_proposal_handler))
            .route("/proposals/job/{job_id}", web::get().to(handlers::get_team_proposals_handler))
            .route("/revenue/distribute", web::post().to(handlers::create_revenue_distribution_handler))
            .route("/revenue/process/{distribution_id}", web::post().to(handlers::process_revenue_distribution_handler))
    );
}
