// src/user_management/privilege.rs

use crate::models::{Privileges, UserRole};
use sqlx::PgPool;

pub fn get_privileges_for_role(role: UserRole) -> Privileges {
    match role {
        UserRole::Admin => Privileges {
            can_create_users: true,
            can_edit_users: true,
            can_delete_users: true,
            can_view_reports: true,
            can_manage_projects: true,
            can_view_clients: true,
            can_view_freelancers: true,
            can_create_projects: true,
            can_edit_projects: true,
            can_delete_projects: true,
            // Add other admin-specific privileges
        },
        UserRole::Client => Privileges {
            can_create_users: false,
            can_edit_users: false,
            can_delete_users: false,
            can_view_reports: false,
            can_manage_projects: true,
            can_view_clients: false,
            can_view_freelancers: true,
            can_create_projects: true,
            can_edit_projects: true,
            can_delete_projects: false,
            // Add other client-specific privileges
        },
        UserRole::Freelancer => Privileges {
            can_create_users: false,
            can_edit_users: false,
            can_delete_users: false,
            can_view_reports: false,
            can_manage_projects: false,
            can_view_clients: true,
            can_view_freelancers: false,
            can_create_projects: false,
            can_edit_projects: false,
            can_delete_projects: false,
            // Add other freelancer-specific privileges
        },
    }
}

pub async fn check_user_privilege(
    pool: &PgPool,
    user_id: i32,
    privilege_name: &str,
) -> Result<bool, String> {
    // Fetch the user's role and privileges from the database
    let user = sqlx::query_as!(
        crate::models::User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    let privileges = get_privileges_for_role(user.role);
    match privilege_name {
        "can_create_users" => Ok(privileges.can_create_users),
        "can_edit_users" => Ok(privileges.can_edit_users),
        "can_delete_users" => Ok(privileges.can_delete_users),
        "can_view_reports" => Ok(privileges.can_view_reports),
        "can_manage_projects" => Ok(privileges.can_manage_projects),
        "can_view_clients" => Ok(privileges.can_view_clients),
        "can_view_freelancers" => Ok(privileges.can_view_freelancers),
        "can_create_projects" => Ok(privileges.can_create_projects),
        "can_edit_projects" => Ok(privileges.can_edit_projects),
        "can_delete_projects" => Ok(privileges.can_delete_projects),
        _ => Err(format!("Unknown privilege: {}", privilege_name)),
    }
}