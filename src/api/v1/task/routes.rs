use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .route("", web::post().to(handlers::create_task_handler)) // Create a new task
            .route("/{task_id}", web::put().to(handlers::update_task_handler)) // Update a task
            .route("/{task_id}", web::delete().to(handlers::delete_task_handler)) // Delete a task
            .route("/project/{project_id}", web::get().to(handlers::get_tasks_for_project_handler)), // Get tasks for a project
    )
}