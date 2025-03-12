use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/moderation").route("/message", web::post().to(handlers::moderate_message_handler)));
}