use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/availability")
            .route("/{user_id}", web::put().to(handlers::update_availability_handler)), // Update availability
    );
}