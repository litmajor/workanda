use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contacts")
            .route("", web::post().to(handlers::create_contact_handler)) // Create new contact info
            .route("/{id}", web::get().to(handlers::get_contact_handler)), // Get contact info by ID

    )

 }   