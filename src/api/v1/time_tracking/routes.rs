use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/time-tracking")
            .route("/create", web::post().to(handlers::create_time_entry_handler)) // Create a new time entry
            .route("/{user_id}", web::get().to(handlers::get_time_entries_handler)) // Get all time entries for a user
            .route("/{user_id}/timesheet", web::get().to(handlers::generate_timesheet_handler)) // Generate a timesheet
            .route("/{user_id}/report", web::get().to(handlers::generate_time_tracking_report_handler)), // Generate a time tracking report

    )      
}