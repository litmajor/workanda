use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/users", web::get().to(handlers::get_all_users))
            .route("/approve-job/{job_id}", web::post().to(handlers::approve_job))
            .route("/escalate-dispute/{dispute_id}", web::post().to(handlers::escalate_dispute))
            .route("/resolve-dispute/{dispute_id}", web::post().to(handlers::resolve_dispute)),
    );
}