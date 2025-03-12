use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/task-lists")
            .route("", web::post().to(handlers::create_task_list_handler)) // Create a new task list
            .route("/project/{project_id}", web::get().to(handlers::get_task_lists_for_project_handler)), // Get task lists for a project
     
     )
    }     