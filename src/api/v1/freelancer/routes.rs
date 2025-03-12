use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/freelancer")
            .route("/profile", web::get().to(handlers::get_freelancer_profile))
            .route("/update-profile", web::put().to(handlers::update_freelancer_profile))
            .route("/jobs", web::get().to(handlers::get_jobs_for_freelancer))
            .route("/submit-proposal/{job_id}", web::post().to(handlers::submit_proposal)),
    );
}