use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/interactions").route("/log", web::post().to(handlers::log_interaction_handler)));
}