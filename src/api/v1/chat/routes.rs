use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chat")
            .route("/create-room", web::post().to(handlers::create_chat_room_handler))
            .route("/join/{room_id}", web::post().to(handlers::join_chat_room_handler)),
    );
}