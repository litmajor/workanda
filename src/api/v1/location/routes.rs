use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/locations")
            .route("", web::post().to(handlers::create_location_handler)) // Create a new location
            .route("", web::get().to(handlers::get_all_locations_handler)), // Get all locations

    )
 }