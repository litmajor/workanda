use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/client")
            .route("/profile", web::get().to(handlers::get_client_profile))
            .route("/update-profile", web::put().to(handlers::update_client_profile))
            .route("/create-job", web::post().to(handlers::create_job))
            .route("/jobs", web::get().to(handlers::get_jobs_for_client)),
    );
}