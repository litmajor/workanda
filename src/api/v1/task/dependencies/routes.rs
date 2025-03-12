use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dependencies")
            .route("", web::post().to(handlers::add_dependency_handler)) // Add a dependency
            .route("", web::delete().to(handlers::remove_dependency_handler)), // Remove a dependency

    )            
}