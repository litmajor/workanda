use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/block")
            .route("/{blocker_id}", web::post().to(handlers::block_user_handler))
            .route("/{blocker_id}/unblock", web::post().to(handlers::unblock_user_handler)),
    );
}