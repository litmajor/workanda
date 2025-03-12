use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reviews")
            .route("", web::post().to(handlers::add_review_handler)) // Add a new review
            .route("/{freelancer_id}", web::get().to(handlers::get_reviews_for_freelancer_endpoint)) // Get all reviews for a freelancer
            .route("/{freelancer_id}/paginated", web::get().to(handlers::get_paginated_reviews_handler)) // Get paginated reviews
            .route("/aggregate/{freelancer_id}", web::get().to(handlers::get_aggregate_ratings)) // Get aggregate ratings
            .route("/{review_id}", web::put().to(handlers::update_review_handler)) // Update a review
            .route("/{review_id}", web::delete().to(handlers::delete_review_handler)), // Delete a review
        )
    }