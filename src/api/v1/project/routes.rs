use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .route("", web::post().to(handlers::create_project_handler)) // Create a new project
            .route("", web::get().to(handlers::get_all_projects_handler)) // Get all projects
            .route("/{project_id}", web::get().to(handlers::get_project_by_id_handler)) // Get a specific project
            .route("/{project_id}", web::put().to(handlers::update_project_handler)) // Update a project
            .route("/{project_id}", web::delete().to(handlers::delete_project_handler)) // Delete a project
            .route("/{project_id}/add-freelancer/{freelancer_id}", web::post().to(handlers::add_freelancer_to_project_handler)) // Add a freelancer to a project
            .route("/{project_id}/remove-freelancer/{freelancer_id}", web::delete().to(handlers::remove_freelancer_from_project_handler)), // Remove a freelancer from a project
      )
    
  }