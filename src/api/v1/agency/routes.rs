
use actix_web::web;
use super::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/agencies")
            .route("", web::post().to(handlers::create_agency_handler))
            .route("/{agency_id}", web::get().to(handlers::get_agency_handler))
            .route("/{agency_id}", web::put().to(handlers::update_agency_handler))
            .route("/{agency_id}/teams/{team_id}", web::post().to(handlers::add_team_to_agency_handler))
            .route("/{agency_id}/teams/{team_id}", web::delete().to(handlers::remove_team_from_agency_handler))
            .route("/{agency_id}/teams", web::get().to(handlers::get_agency_teams_handler))
    );
}
