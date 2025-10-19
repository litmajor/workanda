pub mod auth;
pub mod user;
pub mod health;
pub mod user;
pub mod project;
pub mod job;
pub mod proposal;
pub mod contract;
pub mod escrow;
pub mod messages;
pub mod freelancer;
pub mod client;
pub mod milestone;
pub mod payment;
pub mod admin;
pub mod review;
pub mod team;
pub mod agency;
pub mod ai;
pub mod predictive;
pub mod trust;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::routes::config);
    cfg.service(user::routes::config);
    cfg.service(project::routes::config);
    cfg.service(contract::routes::config);
    cfg.service(escrow::routes::config);
    cfg.service(messages::routes::config);
    cfg.service(freelancer::routes::config);
    cfg.service(client::routes::config);
    cfg.service(milestone::routes::config);
    cfg.service(job::routes::config);
    cfg.service(proposal::routes::config);
    cfg.service(review::routes::config);
    team::routes::config(cfg);
    agency::routes::config(cfg);
    ai::routes::config(cfg);
    predictive::routes::config(cfg);
}

pub use self::auth::*;
pub use self::user::*;
pub use self::health::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/proposals")
            .route("/{job_id}", web::post().to(proposal::submit_proposal_handler))
            .route("/{job_id}", web::get().to(proposal::get_proposals_handler))
            .route("/{proposal_id}/select", web::post().to(proposal::select_proposal_handler))
            .route("/{proposal_id}", web::put().to(proposal::update_proposal_handler))
            .route("/{proposal_id}", web::delete().to(proposal::delete_proposal_handler)),
    );

    cfg.service(
        web::scope("/escrow")
            .route("/{contract_id}", web::post().to(escrow::create_escrow_account_handler))
            .route("/{escrow_id}", web::get().to(escrow::get_escrow_account_handler))
            .route("/{escrow_id}/release", web::post().to(escrow::release_escrow_handler)),
    );


    cfg.service(
        web::scope("/payments")
            .route("/{contract_id}", web::post().to(payment::create_payment_handler))
            .route("/{contract_id}", web::get().to(payment::get_payments_handler))
            .route("/{payment_id}/status", web::put().to(payment::update_payment_status_handler)),
            
    );

    cfg.service(
        web::scope("/contracts")
            .route("", web::post().to(contract::create_contract_handler))
            .route("/with-escrow", web::post().to(contract::create_contract_with_escrow_handler)),
    )
    .service(
        web::scope("/escrow")
            .route("/refund/{id}", web::post().to(escrow::refund_escrow_handler)),
    )
    .service(
        web::scope("/milestones")
            .route("/{contract_id}/{milestone_id}/complete", web::post().to(milestone::mark_milestone_complete_handler)),
    );

    cfg.service(
        web::scope("/freelancers")
            .route("", web::post().to(freelancer::create_freelancer_account_handler))
            .route("/{id}", web::get().to(freelancer::get_freelancer_account_handler))
            .route("/{id}", web::put().to(freelancer::update_freelancer_account_handler))
            .route("/{id}", web::delete().to(freelancer::delete_freelancer_account_handler)),
    )

    .service(
        web::scope("/contracts")
            .route("/client/{id}", web::get().to(contract::fetch_contracts_for_client)),
    )

    .service(
        web::scope("/payments")
            .route("/reminders", web::post().to(payment::send_payment_reminders_handler)),
    );

    cfg.service(
        web::scope("/clients")
            .route("", web::post().to(client::create_client_account_handler))
            .route("/by-email/{email}", web::get().to(client::get_client_by_email_handler))
            .route("/{id}", web::put().to(client::update_client_account_handler))
            .route("/{id}", web::delete().to(client::delete_client_account_handler)),
    )

    .service(
        web::scope("/freelancers")
            .route("/{id}/reviews", web::get().to(freelancer::get_client_reviews_handler)),
    );

    cfg.service(
        web::scope("/reviews")
            .route("", web::post().to(review::add_review_handler))
            .route("/{id}/response", web::post().to(review::add_review_response_handler))
            .route("/{id}/responses", web::get().to(review::get_responses_handler)),
    )
    .service(
        web::scope("/freelancers/{id}")
            .route("/reviews", web::get().to(review::get_reviews_handler))
            .route("/aggregate_ratings", web::get().to(review::get_aggregate_ratings_handler))
            .route("/reviews", web::get().to(review::get_paginated_reviews_handler)),
    );

    cfg.service(
        web::scope("/membership")
            .wrap(AuthMiddlewareFactory)
            .route("/update", web::post().to(membership::update_membership_handler)),
    );

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .wrap(auth::middleware::AuthMiddlewareFactory) // Apply middleware globally
            .service(freelancer::routes::config) // Freelancer routes
            .service(client::routes::config) // Client routes
            .service(admin::routes::config), // Admin routes (hidden/inaccessible)
    );
}