use actix_web::web;
use crate::admin::handlers::*;
use crate::middleware::auth::authorize_role;
use crate::usermanagement::models::UserRole;

/// Configures all admin-related routes.
pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap_fn(|req, srv| {
                // Authorization middleware: Ensure only Admins can access these routes
                let authorize = authorize_role(UserRole::Admin);
                match authorize(&req) {
                    Ok(_) => srv.call(req),
                    Err(e) => Box::pin(async { Err(e) }),
                }
            })
            .route("/dashboard", web::get().to(get_admin_dashboard_data)) // Fetch dashboard data
            .route("/jobs/statistics", web::get().to(get_job_statistics)) // Get job statistics
            .route("/jobs/recent", web::get().to(get_recent_jobs)) // Get recent jobs
            .route("/jobs/categories", web::get().to(get_job_categories)) // Get job categories

            // Clients Management
            .route("/clients", web::get().to(get_clients)) // List all clients
            .route("/clients/{id}", web::get().to(get_client_profile)) // Get client profile by ID
            .route("/clients/{id}", web::put().to(update_client_profile)) // Update client profile
            .route("/clients/{id}", web::delete().to(delete_client)) // Delete client

            // Freelancers Management
            .route("/freelancers", web::get().to(get_freelancers)) // List all freelancers
            .route("/freelancers/{id}", web::get().to(get_freelancer_profile)) // Get freelancer profile by ID
            .route("/freelancers/{id}", web::put().to(update_freelancer_profile)) // Update freelancer profile
            .route("/freelancers/{id}", web::delete().to(delete_freelancer)) // Delete freelancer

            // Active Projects
            .route("/active-projects", web::get().to(get_active_projects)) // Get active projects
            .route("/projects/{id}", web::get().to(get_project_details)) // Get project details by ID

            // Membership Tiers
            .route("/membership-tiers", web::post().to(create_membership_tier)) // Create new membership tier
            .route("/membership-tiers/{id}", web::put().to(update_membership_tier)) // Update membership tier
            .route("/membership-tiers/{id}", web::delete().to(delete_membership_tier)) // Delete membership tier
            .route("/membership-tiers/statistics", web::get().to(get_membership_statistics)) // Get membership statistics
            .route("/membership-tiers/user-distribution", web::get().to(get_user_distribution)) // Get user distribution by tier
            .route("/membership-tiers/revenue", web::get().to(get_revenue_by_tier)) // Get revenue by tier

            // Notifications
            .route("/system-notifications", web::get().to(get_system_notifications)) // Get system notifications
            .route("/system-notifications/{id}/read", web::put().to(mark_notification_read)) // Mark system notification as read
            .route("/system-notifications", web::post().to(create_system_notification)) // Create new system notification
            .route("/user-notifications", web::get().to(get_user_notifications)) // Get user notifications
            .route("/user-notifications/{id}/read", web::put().to(mark_user_notification_read)) // Mark user notification as read
            .route("/user-notifications", web::post().to(create_user_notification)) // Create new user notification

            // Reports
            .route("/reports/generate", web::post().to(generate_custom_report)) // Generate custom reports
            .route("/reports/export", web::post().to(export_report)) // Export reports in CSV format

            // User Activity Logs
            .route("/activity-logs", web::get().to(get_activity_logs)) // Get activity logs with filters

            // User Search
            .route("/search-users", web::get().to(search_users)) // Search users by username, email, role, etc.

            // User Data Export
            .route("/users/{id}/export", web::get().to(export_user_data)) // Export user data in JSON/CSV format

            // Project Management
            .route("/projects/{id}", web::delete().to(delete_project)) // Delete project

            // WebSocket Dashboard
            .route("/dashboard/ws", web::get().to(dashboard_ws)) // Real-time dashboard updates via WebSocket
    );
}

// Example: Pagination and Filtering Query Parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: usize,
    pub per_page: usize,
}

#[derive(Debug, Deserialize)]
pub struct UserSearchQuery {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
    pub registered_after: Option<chrono::DateTime<chrono::Utc>>,
}