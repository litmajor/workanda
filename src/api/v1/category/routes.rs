use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::post().to(handlers::create_category_handler)) // Create a new category
            .route("", web::get().to(handlers::get_categories_handler)) // Get all categories
            .route("/{freelancer_id}/assign", web::post().to(handlers::assign_category_handler)) // Assign a category to a freelancer
            .route("/{freelancer_id}/remove", web::delete().to(handlers::remove_category_handler)) // Remove a category from a freelancer
            .route("/{freelancer_id}", web::get().to(handlers::get_freelancer_categories_handler)) // Get categories for a specific freelancer
            .route("/{category_id}/salary-insight", web::get().to(handlers::get_salary_insight_handler)), // Get salary insights for a category
    )

}

