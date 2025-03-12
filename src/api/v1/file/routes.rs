use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/files").route("/upload", web::post().to(handlers::upload_file_handler)));
}