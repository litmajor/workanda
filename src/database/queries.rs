use sqlx::PgPool;
use crate::models::escrow::{EscrowAccount, NewEscrowAccount};
use crate::models::project::{Project, NewProject};
use crate::models::user::{User, NewUser};
use crate::models::{certification::Certification, education::Education};
use crate::models::{
    financial_report::FinancialReport,
    budget::Budget,
    expense::Expense,
    revenue::Revenue,
};
use crate::models::{budget_category::BudgetCategory, expense::Expense};
use crate::models::{time_tracking_report::TimeTrackingReport, timesheet::Timesheet};
use crate::models::{timesheet::Timesheet, time_entry::TimeEntry};
use crate::models::{time_entry::TimeEntry, user::User, task::Task};
use crate::models::interaction::Interaction;
use crate::models::chat_room::{ChatRoom, RoomMember};
use crate::models::blocked_user::BlockedUser;
use crate::models::file_entry::FileEntry;
use crate::models::financial_goal::FinancialGoal;
use crate::models::expense_transaction::ExpenseTransaction;
use crate::models::expense_category::ExpenseCategory;
use crate::models::savings_goal::SavingsGoal;
use crate::models::planned_expense::PlannedExpense;
use crate::models::income_source::IncomeSource;
use crate::models::*;
use crate::models::reviews::ClientReview;
use crate::models::reviews::{Review, ClientReviewInput, UpdatedReview, AggregateRatings, PaginationParams};
use crate::models::{ProjectTemplate, Project};






pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, hashed_password)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(&new_user.username)
    .bind(&new_user.email)
    .bind(&new_user.hashed_password)
    .fetch_one(pool)
    .await
}


pub async fn find_user_by_username(pool: &Pool, username: &str) -> Result<Option<User>, sqlx::Error> {

    sqlx::query_as!(
    
    User,
    
    "SELECT * FROM users WHERE username = $1",
    
    username
    
    )
    
    .fetch_optional(pool)
    
    .await
    
    }


pub async fn create_project(pool: &PgPool, new_project: NewProject) -> Result<Project, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (name, description, client_id, freelancer_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(new_project.name)
    .bind(new_project.description)
    .bind(new_project.client_id)
    .bind(new_project.freelancer_id)
    .fetch_one(pool)
    .await
}


// Update a project
pub async fn update_project(
    pool: &PgPool,
    project_id: i32,
    updated_project: UpdatedProject,
) -> Result<Project, sqlx::Error> {
    let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
        "UPDATE projects SET ",
    );

    if let Some(name) = &updated_project.name {
        query_builder.push("name = ").push_bind(name).push(", ");
    }
    if let Some(budget) = updated_project.budget {
        query_builder.push("budget = ").push_bind(budget).push(", ");
    }
    if let Some(client_id) = updated_project.client_id {
        query_builder.push("client_id = ").push_bind(client_id).push(", ");
    }
    if let Some(freelancer_id) = updated_project.freelancer_id {
        query_builder.push("freelancer_id = ").push_bind(freelancer_id).push(", ");
    }
    if let Some(category) = &updated_project.category {
        query_builder.push("category = ").push_bind(category).push(", ");
    }
    if let Some(description) = &updated_project.description {
        query_builder.push("description = ").push_bind(description).push(", ");
    }
    if let Some(status) = &updated_project.status {
        query_builder.push("status = ").push_bind(status.to_string()).push(", ");
    }

    query_builder.push("updated_at = NOW() WHERE id = ").push_bind(project_id).push(" RETURNING *");

    let query = query_builder.build();
    query.fetch_one(pool).await
}

// Delete a project
pub async fn delete_project(
    pool: &PgPool,
    project_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(project_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}


// Add a freelancer to a project
pub async fn add_freelancer_to_project(
    pool: &PgPool,
    project_id: i32,
    freelancer_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO project_members (project_id, freelancer_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(project_id)
    .bind(freelancer_id)
    .execute(pool)
    .await?;

    Ok(())
}

// Remove a freelancer from a project
pub async fn remove_freelancer_from_project(
    pool: &PgPool,
    project_id: i32,
    freelancer_id: Uuid,
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM project_members WHERE project_id = $1 AND freelancer_id = $2")
        .bind(project_id)
        .bind(freelancer_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

pub async fn create_escrow_account(
    pool: &PgPool,
    new_account: NewEscrowAccount,
) -> Result<EscrowAccount, sqlx::Error> {
    sqlx::query_as::<_, EscrowAccount>(
        r#"
        INSERT INTO escrow_accounts (sender_id, receiver_id, amount, currency, release_conditions)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(new_account.sender_id)
    .bind(new_account.receiver_id)
    .bind(new_account.amount)
    .bind(new_account.currency)
    .bind(new_account.release_conditions)
    .fetch_one(pool)
    .await
}

pub async fn get_escrow_account_by_id(
    pool: &PgPool,
    account_id: i32,
) -> Result<Option<EscrowAccount>, sqlx::Error> {
    sqlx::query_as::<_, EscrowAccount>("SELECT * FROM escrow_accounts WHERE id = $1")
        .bind(account_id)
        .fetch_optional(pool)
        .await
}


pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, hashed_password, profile_picture)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(new_user.username)
    .bind(new_user.email)
    .bind(new_user.hashed_password)
    .bind(new_user.profile_picture)
    .fetch_one(pool)
    .await
}

pub async fn register_user(
    pool: &Pool,
    username: &str,
    email: &str,
    password: &str,
    role_name: Option<&str>,
) -> Result<User, Box<dyn Error>> {
    let mut tx = pool.begin().await?;
    let hashed_password = hash_password(password).await?;

    let role_id = if let Some(name) = role_name {
        sqlx::query_scalar!(
            "SELECT id FROM roles WHERE name = $1",
            name
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or("Role not found")?
    } else {
        None
    };

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, hashed_password, role_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        username,
        email,
        hashed_password,
        role_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn get_clients(
    pool: &Pool,
    page: u32,
    per_page: u32,
    filters: Option<ClientFilter>,
) -> Result<Vec<ClientAccount>, sqlx::Error> {
    let offset = (page - 1) * per_page;
    let mut query = "SELECT * FROM client_accounts".to_string();
    let mut params: Vec<String> = Vec::new();
    let mut conditions = Vec::new();

    if let Some(filters) = filters {
        if let Some(name) = filters.name {
            conditions.push("name LIKE $1");
            params.push(format!("%{}%", name));
        }
        if let Some(email) = filters.email {
            conditions.push("email = $2");
            params.push(email);
        }
        // Add other filters...
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY created_at DESC LIMIT $3 OFFSET $4");
    params.push(per_page.to_string());
    params.push(offset.to_string());

    sqlx::query_as::<_, ClientAccount>(&query)
        .bind(params)
        .fetch_all(pool)
        .await
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "membership_tier")]
pub enum MembershipTier {
    Basic,
    Normal,
    Premium,
    Enterprise,
}

// Use directly in queries:
sqlx::query!(
    "UPDATE users SET membership = $1 WHERE id = $2",
    MembershipTier::Premium as MembershipTier,
    user_id
);

pub async fn send_message(
    pool: &Pool,
    sender_id: i32,
    recipient_id: i32,
    content: &str,
) -> Result<Message, sqlx::Error> {
    sqlx::query_as!(
        Message,
        r#"
        INSERT INTO messages (sender_id, recipient_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        sender_id,
        recipient_id,
        content
    )
    .fetch_one(pool)
    .await
}

//================== Task Queries ==================
pub async fn get_tasks_for_project(
    pool: &PgPool,
    project_id: i32,
) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE project_id = $1")
        .bind(project_id)
        .fetch_all(pool)
        .await
}

pub async fn create_task(pool: &PgPool, new_task: NewTask) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (project_id, title, description, due_date, priority, status)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(new_task.project_id)
    .bind(&new_task.title)
    .bind(&new_task.description)
    .bind(new_task.due_date)
    .bind(new_task.priority.to_string())
    .bind(new_task.status.to_string())
    .fetch_one(pool)
    .await
}

pub async fn update_task(
    pool: &PgPool,
    task_id: i32,
    updated_task: Task,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET title = COALESCE($2, title),
            description = COALESCE($3, description),
            due_date = COALESCE($4, due_date),
            priority = COALESCE($5, priority),
            status = COALESCE($6, status)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(task_id)
    .bind(&updated_task.title)
    .bind(&updated_task.description)
    .bind(updated_task.due_date)
    .bind(updated_task.priority.to_string())
    .bind(updated_task.status.to_string())
    .fetch_one(pool)
    .await
}


// Get all tasks for a specific project
pub async fn get_tasks_for_project(
    pool: &PgPool,
    project_id: i32,
) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE project_id = $1")
        .bind(project_id)
        .fetch_all(pool)
        .await
}

pub async fn delete_task(pool: &PgPool, task_id: i32) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(task_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

// ================== Escrow Queries ==================
pub async fn create_escrow_account(
    pool: &PgPool,
    new_account: NewEscrowAccount,
) -> Result<EscrowAccount, sqlx::Error> {
    sqlx::query_as::<_, EscrowAccount>(
        r#"
        INSERT INTO escrow_accounts (sender_id, receiver_id, amount, currency, status, release_conditions)
        VALUES ($1, $2, $3, $4, 'pending', $5)
        RETURNING *
        "#,
    )
    .bind(new_account.sender_id)
    .bind(new_account.receiver_id)
    .bind(new_account.amount)
    .bind(&new_account.currency)
    .bind(&new_account.release_conditions)
    .fetch_one(pool)
    .await
}

pub async fn get_escrow_account_by_id(
    pool: &PgPool,
    account_id: i32,
) -> Result<Option<EscrowAccount>, sqlx::Error> {
    sqlx::query_as::<_, EscrowAccount>("SELECT * FROM escrow_accounts WHERE id = $1")
        .bind(account_id)
        .fetch_optional(pool)
        .await
}

pub async fn update_escrow_status(
    pool: &PgPool,
    account_id: i32,
    new_status: String,
) -> Result<EscrowAccount, sqlx::Error> {
    sqlx::query_as::<_, EscrowAccount>(
        r#"
        UPDATE escrow_accounts
        SET status = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(account_id)
    .bind(new_status)
    .fetch_one(pool)
    .await
}

pub async fn delete_escrow_account(
    pool: &PgPool,
    account_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM escrow_accounts WHERE id = $1")
        .bind(account_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}


pub async fn release_escrow_funds(
    pool: &PgPool,
    escrow_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE escrow_accounts
        SET status = 'released', released_at = NOW()
        WHERE id = $1 AND status = 'completed'
        "#,
    )
    .bind(escrow_id)
    .execute(pool)
    .await
    .map(|res| res.rows_affected())
}

pub async fn refund_escrow_funds(
    pool: &PgPool,
    escrow_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE escrow_accounts
        SET status = 'refunded', refunded_at = NOW()
        WHERE id = $1 AND status = 'pending'
        "#,
    )
    .bind(escrow_id)
    .execute(pool)
    .await
    .map(|res| res.rows_affected())
}


// ================== Contract Queries ==================
pub async fn create_contract(
    pool: &PgPool,
    new_contract: NewContract,
) -> Result<Contract, sqlx::Error> {
    sqlx::query_as::<_, Contract>(
        r#"
        INSERT INTO contracts (client_id, freelancer_id, title, description, value, start_date, end_date, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending')
        RETURNING *
        "#,
    )
    .bind(new_contract.client_id)
    .bind(new_contract.freelancer_id)
    .bind(&new_contract.title)
    .bind(&new_contract.description)
    .bind(new_contract.value)
    .bind(new_contract.start_date)
    .bind(new_contract.end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_contract_by_id(
    pool: &PgPool,
    contract_id: i32,
) -> Result<Option<Contract>, sqlx::Error> {
    sqlx::query_as::<_, Contract>("SELECT * FROM contracts WHERE id = $1")
        .bind(contract_id)
        .fetch_optional(pool)
        .await
}

pub async fn update_contract(
    pool: &PgPool,
    contract_id: i32,
    updated_contract: Contract,
) -> Result<Contract, sqlx::Error> {
    sqlx::query_as::<_, Contract>(
        r#"
        UPDATE contracts
        SET title = COALESCE($2, title),
            description = COALESCE($3, description),
            value = COALESCE($4, value),
            start_date = COALESCE($5, start_date),
            end_date = COALESCE($6, end_date),
            status = COALESCE($7, status)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(contract_id)
    .bind(&updated_contract.title)
    .bind(&updated_contract.description)
    .bind(updated_contract.value)
    .bind(updated_contract.start_date)
    .bind(updated_contract.end_date)
    .bind(updated_contract.status.to_string())
    .fetch_one(pool)
    .await
}

pub async fn delete_contract(pool: &PgPool, contract_id: i32) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM contracts WHERE id = $1")
        .bind(contract_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

// ================== Milestone Queries ==================
pub async fn add_milestone(
    pool: &PgPool,
    new_milestone: NewMilestone,
) -> Result<Milestone, sqlx::Error> {
    sqlx::query_as::<_, Milestone>(
        r#"
        INSERT INTO milestones (contract_id, project_id, description, due_date, payment_amount, budget)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(new_milestone.contract_id)
    .bind(new_milestone.project_id)
    .bind(&new_milestone.description)
    .bind(new_milestone.due_date)
    .bind(new_milestone.payment_amount)
    .bind(new_milestone.budget)
    .fetch_one(pool)
    .await
}

pub async fn get_milestones_for_contract(
    pool: &PgPool,
    contract_id: i32,
) -> Result<Vec<Milestone>, sqlx::Error> {
    sqlx::query_as::<_, Milestone>("SELECT * FROM milestones WHERE contract_id = $1")
        .bind(contract_id)
        .fetch_all(pool)
        .await
}

pub async fn update_milestone(
    pool: &PgPool,
    milestone_id: i32,
    updated_milestone: MilestoneUpdate,
) -> Result<Milestone, sqlx::Error> {
    sqlx::query_as::<_, Milestone>(
        r#"
        UPDATE milestones
        SET description = COALESCE($2, description),
            due_date = COALESCE($3, due_date),
            payment_amount = COALESCE($4, payment_amount),
            completion_status = COALESCE($5, completion_status),
            budget = COALESCE($6, budget)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(milestone_id)
    .bind(&updated_milestone.description)
    .bind(updated_milestone.due_date)
    .bind(updated_milestone.payment_amount)
    .bind(updated_milestone.completion_status)
    .bind(updated_milestone.budget)
    .fetch_one(pool)
    .await
}

pub async fn mark_milestone_complete(
    pool: &PgPool,
    milestone_id: i32,
) -> Result<Milestone, sqlx::Error> {
    sqlx::query_as::<_, Milestone>(
        r#"
        UPDATE milestones
        SET completion_status = TRUE, payment_released = TRUE, updated_at = NOW()
        WHERE id = $1 AND completion_status = FALSE
        RETURNING *
        "#,
    )
    .bind(milestone_id)
    .fetch_one(pool)
    .await
}

// ================== Payment Queries ==================
pub async fn create_payment(
    pool: &PgPool,
    new_payment: CreateTransactionRequest,
) -> Result<Transaction, sqlx::Error> {
    let transaction_type = new_payment.transaction_type.to_string();
    sqlx::query_as::<_, Transaction>(
        r#"
        INSERT INTO transactions (escrow_account_id, sender_id, receiver_id, amount, status, fee, custom_data, transaction_type, created_at, description, project_id)
        VALUES ($1, $2, $3, $4, 'pending', 0.0, '{}', $5, NOW(), $6, $7)
        RETURNING *
        "#,
    )
    .bind(new_payment.project_id)
    .bind(new_payment.sender_id)
    .bind(new_payment.receiver_id)
    .bind(new_payment.amount)
    .bind(transaction_type)
    .bind(new_payment.description.unwrap_or_default())
    .bind(new_payment.project_id)
    .fetch_one(pool)
    .await
}

pub async fn get_payments_for_contract(
    pool: &PgPool,
    contract_id: i32,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE contract_id = $1")
        .bind(contract_id)
        .fetch_all(pool)
        .await
}


pub async fn update_payment_status(
    pool: &PgPool,
    payment_id: i32,
    new_status: String,
) -> Result<Transaction, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        r#"
        UPDATE transactions
        SET status = $2, updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(payment_id)
    .bind(new_status)
    .fetch_one(pool)
    .await
}



// ================== Job Queries ==================
pub async fn create_job(pool: &PgPool, new_job: NewJob) -> Result<Job, sqlx::Error> {
    sqlx::query_as::<_, Job>(
        r#"
        INSERT INTO jobs (name, budget, client_id, freelancer_id, start_date, end_date, category, description)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(&new_job.name)
    .bind(new_job.budget)
    .bind(new_job.client_id)
    .bind(new_job.freelancer_id)
    .bind(new_job.start_date)
    .bind(new_job.end_date)
    .bind(&new_job.category)
    .bind(&new_job.description)
    .fetch_one(pool)
    .await
}

pub async fn get_jobs_for_client(
    pool: &PgPool,
    client_id: i32,
) -> Result<Vec<Job>, sqlx::Error> {
    sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE client_id = $1")
        .bind(client_id)
        .fetch_all(pool)
        .await
}

pub async fn get_job_by_id(pool: &PgPool, job_id: i32) -> Result<Option<Job>, sqlx::Error> {
    sqlx::query_as::<_, Job>("SELECT * FROM jobs WHERE id = $1")
        .bind(job_id)
        .fetch_optional(pool)
        .await
}

pub async fn update_job(pool: &PgPool, job_id: i32, updated_job: Job) -> Result<Job, sqlx::Error> {
    sqlx::query_as::<_, Job>(
        r#"
        UPDATE jobs
        SET name = COALESCE($2, name),
            budget = COALESCE($3, budget),
            start_date = COALESCE($4, start_date),
            end_date = COALESCE($5, end_date),
            category = COALESCE($6, category),
            description = COALESCE($7, description),
            status = COALESCE($8, status)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(job_id)
    .bind(&updated_job.name)
    .bind(updated_job.budget)
    .bind(updated_job.start_date)
    .bind(updated_job.end_date)
    .bind(&updated_job.category)
    .bind(&updated_job.description)
    .bind(&updated_job.status.to_string())
    .fetch_one(pool)
    .await
}

pub async fn delete_job(pool: &PgPool, job_id: i32) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM jobs WHERE id = $1")
        .bind(job_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

// ================== Proposal Queries ==================
pub async fn submit_proposal(
    pool: &PgPool,
    job_id: i32,
    new_proposal:NewProposal,
) -> Result<Proposal, sqlx::Error> {
    sqlx::query_as::<_, Proposal>(
        r#"
        INSERT INTO proposals (job_id, freelancer_id, bid_amount, message, status)
        VALUES ($1, $2, $3, $4, 'pending')
        RETURNING *
        "#,
    )
    .bind(job_id)
    .bind(new_proposal.freelancer_id)
    .bind(new_proposal.bid_amount)
    .bind(&new_proposal.message)
    .fetch_one(pool)
    .await
}

pub async fn get_proposals_for_job(
    pool: &PgPool,
    job_id: i32,
) -> Result<Vec<Proposal>, sqlx::Error> {
    sqlx::query_as::<_, Proposal>("SELECT * FROM proposals WHERE job_id = $1")
        .bind(job_id)
        .fetch_all(pool)
        .await
}

pub async fn select_proposal(
    pool: &PgPool,
    proposal_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE proposals
        SET status = 'selected'
        WHERE id = $1 AND status = 'pending'
        "#,
    )
    .bind(proposal_id)
    .execute(pool)
    .await?;

    Ok(())
}




pub async fn create_income_source(
    pool: &PgPool,
    new_income: NewIncomeSource,
) -> Result<IncomeSource, sqlx::Error> {
    sqlx::query_as::<_, IncomeSource>(
        r#"
        INSERT INTO income_sources (user_id, name, amount, date, details)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(new_income.user_id)
    .bind(&new_income.name)
    .bind(new_income.amount)
    .bind(new_income.date)
    .bind(new_income.details)
    .fetch_one(pool)
    .await
}

pub async fn get_income_sources_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<IncomeSource>, sqlx::Error> {
    sqlx::query_as::<_, IncomeSource>("SELECT * FROM income_sources WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
}


pub async fn create_planned_expense(
    pool: &PgPool,
    new_expense: NewPlannedExpense,
) -> Result<PlannedExpense, sqlx::Error> {
    sqlx::query_as::<_, PlannedExpense>(
        r#"
        INSERT INTO planned_expenses (user_id, name, amount, due_date)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(new_expense.user_id)
    .bind(&new_expense.name)
    .bind(new_expense.amount)
    .bind(new_expense.due_date)
    .fetch_one(pool)
    .await
}

pub async fn get_planned_expenses_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<PlannedExpense>, sqlx::Error> {
    sqlx::query_as::<_, PlannedExpense>("SELECT * FROM planned_expenses WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn create_savings_goal(
    pool: &PgPool,
    new_goal: NewSavingsGoal,
) -> Result<SavingsGoal, sqlx::Error> {
    sqlx::query_as::<_, SavingsGoal>(
        r#"
        INSERT INTO savings_goals (user_id, name, target_amount, deadline)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(new_goal.user_id)
    .bind(&new_goal.name)
    .bind(new_goal.target_amount)
    .bind(new_goal.deadline)
    .fetch_one(pool)
    .await
}

pub async fn get_savings_goals_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<SavingsGoal>, sqlx::Error> {
    sqlx::query_as::<_, SavingsGoal>("SELECT * FROM savings_goals WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn get_expense_categories(
    pool: &PgPool,
) -> Result<Vec<ExpenseCategory>, sqlx::Error> {
    sqlx::query_as::<_, ExpenseCategory>("SELECT * FROM expense_categories")
        .fetch_all(pool)
        .await
}

pub async fn create_expense_transaction(
    pool: &PgPool,
    new_expense: ExpenseTransaction,
) -> Result<ExpenseTransaction, sqlx::Error> {
    sqlx::query_as::<_, ExpenseTransaction>(
        r#"
        INSERT INTO expense_transactions (user_id, category_id, subcategory_id, amount, date, description)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(new_expense.user_id)
    .bind(new_expense.category_id)
    .bind(new_expense.subcategory_id)
    .bind(new_expense.amount)
    .bind(new_expense.date)
    .bind(new_expense.description)
    .fetch_one(pool)
    .await
}

pub async fn get_expense_transactions_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<ExpenseTransaction>, sqlx::Error> {
    sqlx::query_as::<_, ExpenseTransaction>(
        "SELECT * FROM expense_transactions WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn create_financial_goal(
    pool: &PgPool,
    new_goal: FinancialGoal,
) -> Result<FinancialGoal, sqlx::Error> {
    sqlx::query_as::<_, FinancialGoal>(
        r#"
        INSERT INTO financial_goals (name, target_amount, target_date, current_amount)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&new_goal.name)
    .bind(new_goal.target_amount)
    .bind(new_goal.target_date)
    .bind(new_goal.current_amount)
    .fetch_one(pool)
    .await
}

pub async fn get_financial_goals_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<FinancialGoal>, sqlx::Error> {
    sqlx::query_as::<_, FinancialGoal>(
        "SELECT * FROM financial_goals WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn upload_file(
    pool: &PgPool,
    new_file: NewFileEntry,
) -> Result<FileEntry, sqlx::Error> {
    sqlx::query_as::<_, FileEntry>(
        r#"
        INSERT INTO file_entries (filename, url)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(&new_file.filename)
    .bind(&new_file.url)
    .fetch_one(pool)
    .await
}

pub async fn get_files_for_project(
    pool: &PgPool,
    project_id: i32,
) -> Result<Vec<FileEntry>, sqlx::Error> {
    sqlx::query_as::<_, FileEntry>("SELECT * FROM file_entries WHERE project_id = $1")
        .bind(project_id)
        .fetch_all(pool)
        .await
}

pub async fn block_user(
    pool: &PgPool,
    blocker_id: i32,
    blocked_id: i32,
) -> Result<BlockedUser, sqlx::Error> {
    sqlx::query_as::<_, BlockedUser>(
        r#"
        INSERT INTO blocked_users (blocker_id, blocked_id, created_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT DO NOTHING
        RETURNING *
        "#,
    )
    .bind(blocker_id)
    .bind(blocked_id)
    .fetch_one(pool)
    .await
}

pub async fn unblock_user(
    pool: &PgPool,
    blocker_id: i32,
    blocked_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM blocked_users WHERE blocker_id = $1 AND blocked_id = $2")
        .bind(blocker_id)
        .bind(blocked_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}


pub async fn create_chat_room(
    pool: &PgPool,
    new_room: ChatRoom,
) -> Result<ChatRoom, sqlx::Error> {
    sqlx::query_as::<_, ChatRoom>(
        r#"
        INSERT INTO chat_rooms (name, is_private, created_at, created_by)
        VALUES ($1, $2, NOW(), $3)
        RETURNING *
        "#,
    )
    .bind(&new_room.name)
    .bind(new_room.is_private)
    .bind(new_room.created_by)
    .fetch_one(pool)
    .await
}

pub async fn join_chat_room(
    pool: &PgPool,
    room_id: i32,
    user_id: i32,
) -> Result<RoomMember, sqlx::Error> {
    sqlx::query_as::<_, RoomMember>(
        r#"
        INSERT INTO room_members (room_id, user_id, joined_at)
        VALUES ($1, $2, NOW())
        RETURNING *
        "#,
    )
    .bind(room_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_chat_rooms_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<ChatRoom>, sqlx::Error> {
    sqlx::query_as::<_, ChatRoom>(
        r#"
        SELECT cr.*
        FROM chat_rooms cr
        INNER JOIN room_members rm ON cr.id = rm.room_id
        WHERE rm.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}


pub async fn log_interaction(
    pool: &PgPool,
    freelancer_id: &str,
    project_id: &str,
    interaction_type: &str,
) -> Result<Interaction, sqlx::Error> {
    sqlx::query_as::<_, Interaction>(
        r#"
        INSERT INTO interactions (freelancer_id, project_id, interaction_type, interaction_date)
        VALUES ($1, $2, $3, NOW())
        RETURNING *
        "#,
    )
    .bind(freelancer_id)
    .bind(project_id)
    .bind(interaction_type)
    .fetch_one(pool)
    .await
}

pub async fn get_interactions_for_freelancer(
    pool: &PgPool,
    freelancer_id: &str,
) -> Result<Vec<Interaction>, sqlx::Error> {
    sqlx::query_as::<_, Interaction>(
        "SELECT * FROM interactions WHERE freelancer_id = $1",
    )
    .bind(freelancer_id)
    .fetch_all(pool)
    .await
}


pub async fn add_certification(
    pool: &PgPool,
    new_cert: Certification,
) -> Result<Certification, sqlx::Error> {
    sqlx::query_as::<_, Certification>(
        r#"
        INSERT INTO certifications (freelancer_id, title, issuing_organization, date_issued)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(new_cert.freelancer_id)
    .bind(&new_cert.title)
    .bind(&new_cert.issuing_organization)
    .bind(new_cert.date_issued)
    .fetch_one(pool)
    .await
}

pub async fn add_education(
    pool: &PgPool,
    new_edu: Education,
) -> Result<Education, sqlx::Error> {
    sqlx::query_as::<_, Education>(
        r#"
        INSERT INTO educations (freelancer_id, institution, degree, field_of_study, graduation_year)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(new_edu.freelancer_id)
    .bind(&new_edu.institution)
    .bind(&new_edu.degree)
    .bind(&new_edu.field_of_study)
    .bind(new_edu.graduation_year)
    .fetch_one(pool)
    .await
}


// Create a new time entry
pub async fn create_time_entry(
    pool: &PgPool,
    user_id: i32,
    task_id: i32,
    start_time: chrono::DateTime<chrono::Utc>,
    end_time: chrono::DateTime<chrono::Utc>,
) -> Result<TimeEntry, sqlx::Error> {
    sqlx::query_as::<_, TimeEntry>(
        r#"
        INSERT INTO time_entries (user_id, task_id, start_time, end_time)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(task_id)
    .bind(start_time.naive_utc())
    .bind(end_time.naive_utc())
    .fetch_one(pool)
    .await
}

// Get all time entries for a specific user
pub async fn get_time_entries_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<TimeEntry>, sqlx::Error> {
    sqlx::query_as::<_, TimeEntry>(
        "SELECT * FROM time_entries WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

// Get all time entries for a specific task
pub async fn get_time_entries_for_task(
    pool: &PgPool,
    task_id: i32,
) -> Result<Vec<TimeEntry>, sqlx::Error> {
    sqlx::query_as::<_, TimeEntry>(
        "SELECT * FROM time_entries WHERE task_id = $1",
    )
    .bind(task_id)
    .fetch_all(pool)
    .await
}

// Generate a timesheet for a specific user
pub async fn generate_timesheet_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Timesheet, sqlx::Error> {
    let time_entries = sqlx::query_as::<_, TimeEntry>(
        "SELECT * FROM time_entries WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(Timesheet {
        user: User { id: user_id, ..Default::default() }, // Placeholder for user details
        time_entries,
    })
}

// Get total time spent by a user
pub async fn get_total_time_spent_by_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<chrono::Duration, sqlx::Error> {
    let total_seconds: i64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(EXTRACT(EPOCH FROM (end_time - start_time))), 0)
        FROM time_entries
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(chrono::Duration::seconds(total_seconds))
}


// Generate a time tracking report for a specific user
pub async fn generate_time_tracking_report_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<TimeTrackingReport, sqlx::Error> {
    let timesheets = sqlx::query_as::<_, Timesheet>(
        r#"
        SELECT 
            u.id AS user_id, 
            array_agg(te.*) AS time_entries
        FROM users u
        LEFT JOIN time_entries te ON u.id = te.user_id
        WHERE u.id = $1
        GROUP BY u.id
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let report = TimeTrackingReport {
        user: User { id: user_id, ..Default::default() }, // Placeholder for user details
        timesheets,
    };

    Ok(report)
}


// Create a new budget category
pub async fn create_budget_category(
    pool: &PgPool,
    budget_id: i32,
    name: String,
    allocated_amount: f64,
) -> Result<BudgetCategory, sqlx::Error> {
    sqlx::query_as::<_, BudgetCategory>(
        r#"
        INSERT INTO budget_categories (budget_id, name, allocated_amount)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(budget_id)
    .bind(name)
    .bind(allocated_amount)
    .fetch_one(pool)
    .await
}

// Get all budget categories for a specific budget
pub async fn get_budget_categories_for_budget(
    pool: &PgPool,
    budget_id: i32,
) -> Result<Vec<BudgetCategory>, sqlx::Error> {
    sqlx::query_as::<_, BudgetCategory>(
        "SELECT * FROM budget_categories WHERE budget_id = $1",
    )
    .bind(budget_id)
    .fetch_all(pool)
    .await
}

// Update a budget category
pub async fn update_budget_category(
    pool: &PgPool,
    category_id: i32,
    updated_category: BudgetCategory,
) -> Result<BudgetCategory, sqlx::Error> {
    sqlx::query_as::<_, BudgetCategory>(
        r#"
        UPDATE budget_categories
        SET name = COALESCE($2, name),
            allocated_amount = COALESCE($3, allocated_amount)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(category_id)
    .bind(&updated_category.name)
    .bind(updated_category.allocated_amount)
    .fetch_one(pool)
    .await
}

// Delete a budget category
pub async fn delete_budget_category(
    pool: &PgPool,
    category_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM budget_categories WHERE id = $1")
        .bind(category_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

// Create a new expense
pub async fn create_expense(
    pool: &PgPool,
    expense: Expense,
) -> Result<Expense, sqlx::Error> {
    sqlx::query_as::<_, Expense>(
        r#"
        INSERT INTO expenses (budget_id, category_id, subcategory_id, amount, description, datetime, paid, invoice_id, freelancer_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(expense.budget_id)
    .bind(expense.category_id)
    .bind(expense.subcategory_id)
    .bind(expense.amount.amount)
    .bind(&expense.description)
    .bind(expense.datetime)
    .bind(expense.paid)
    .bind(expense.invoice_id)
    .bind(expense.freelancer_id)
    .fetch_one(pool)
    .await
}

// Get all expenses for a specific budget
pub async fn get_expenses_for_budget(
    pool: &PgPool,
    budget_id: i32,
) -> Result<Vec<Expense>, sqlx::Error> {
    sqlx::query_as::<_, Expense>("SELECT * FROM expenses WHERE budget_id = $1")
        .bind(budget_id)
        .fetch_all(pool)
        .await
}

// Update an expense
pub async fn update_expense(
    pool: &PgPool,
    expense_id: i32,
    updated_expense: Expense,
) -> Result<Expense, sqlx::Error> {
    sqlx::query_as::<_, Expense>(
        r#"
        UPDATE expenses
        SET category_id = COALESCE($2, category_id),
            subcategory_id = COALESCE($3, subcategory_id),
            amount = COALESCE($4, amount),
            description = COALESCE($5, description),
            datetime = COALESCE($6, datetime),
            paid = COALESCE($7, paid),
            invoice_id = COALESCE($8, invoice_id),
            freelancer_id = COALESCE($9, freelancer_id)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(expense_id)
    .bind(updated_expense.category_id)
    .bind(updated_expense.subcategory_id)
    .bind(updated_expense.amount.amount)
    .bind(&updated_expense.description)
    .bind(updated_expense.datetime)
    .bind(updated_expense.paid)
    .bind(updated_expense.invoice_id)
    .bind(updated_expense.freelancer_id)
    .fetch_one(pool)
    .await
}

// Delete an expense
pub async fn delete_expense(
    pool: &PgPool,
    expense_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM expenses WHERE id = $1")
        .bind(expense_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

// Create a new revenue entry
pub async fn create_revenue(
    pool: &PgPool,
    budget_id: i32,
    amount: f64,
    datetime_received: chrono::NaiveDateTime,
) -> Result<Revenue, sqlx::Error> {
    sqlx::query_as::<_, Revenue>(
        r#"
        INSERT INTO revenues (budget_id, amount, datetime_received)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(budget_id)
    .bind(amount)
    .bind(datetime_received)
    .fetch_one(pool)
    .await
}

// Get all revenue entries for a specific budget
pub async fn get_revenues_for_budget(
    pool: &PgPool,
    budget_id: i32,
) -> Result<Vec<Revenue>, sqlx::Error> {
    sqlx::query_as::<_, Revenue>("SELECT * FROM revenues WHERE budget_id = $1")
        .bind(budget_id)
        .fetch_all(pool)
        .await
}


// Generate a financial report for a specific budget
pub async fn generate_financial_report_for_budget(
    pool: &PgPool,
    budget_id: i32,
) -> Result<FinancialReport, sqlx::Error> {
    let budget = sqlx::query_as::<_, Budget>(
        "SELECT * FROM budgets WHERE id = $1",
    )
    .bind(budget_id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    let expenses = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses WHERE budget_id = $1",
    )
    .bind(budget_id)
    .fetch_all(pool)
    .await?;

    let revenues = sqlx::query_as::<_, Revenue>(
        "SELECT * FROM revenues WHERE budget_id = $1",
    )
    .bind(budget_id)
    .fetch_all(pool)
    .await?;

    Ok(FinancialReport {
        budget,
        expenses,
        revenues,
    })
}

// Get total spent for a specific budget
pub async fn get_total_spent_for_budget(
    pool: &PgPool,
    budget_id: i32,
) -> Result<f64, sqlx::Error> {
    let total_spent: Option<f64> = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount), 0) FROM expenses WHERE budget_id = $1",
    )
    .bind(budget_id)
    .fetch_one(pool)
    .await?;

    Ok(total_spent.unwrap_or(0.0))
}

// Get remaining budget for a specific budget
pub async fn get_remaining_budget_for_budget(
    pool: &PgPool,
    budget_id: i32,
) -> Result<f64, sqlx::Error> {
    let budget = sqlx::query_as::<_, Budget>(
        "SELECT * FROM budgets WHERE id = $1",
    )
    .bind(budget_id)
    .fetch_optional(pool)
    .await?
    .ok_or(sqlx::Error::RowNotFound)?;

    let total_spent = get_total_spent_for_budget(pool, budget_id).await?;
    Ok(budget.total_allocated.amount - total_spent)
}

// Add a certification for a freelancer
pub async fn add_certification(
    pool: &PgPool,
    freelancer_id: i32,
    title: String,
    issuing_organization: String,
    date_issued: chrono::NaiveDate,
) -> Result<Certification, sqlx::Error> {
    sqlx::query_as::<_, Certification>(
        r#"
        INSERT INTO certifications (freelancer_id, title, issuing_organization, date_issued)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(freelancer_id)
    .bind(title)
    .bind(issuing_organization)
    .bind(date_issued)
    .fetch_one(pool)
    .await
}

// Get all certifications for a freelancer
pub async fn get_certifications_for_freelancer(
    pool: &PgPool,
    freelancer_id: i32,
) -> Result<Vec<Certification>, sqlx::Error> {
    sqlx::query_as::<_, Certification>(
        "SELECT * FROM certifications WHERE freelancer_id = $1",
    )
    .bind(freelancer_id)
    .fetch_all(pool)
    .await
}

// Add education details for a freelancer
pub async fn add_education(
    pool: &PgPool,
    freelancer_id: i32,
    institution: String,
    degree: String,
    field_of_study: String,
    graduation_year: i32,
) -> Result<Education, sqlx::Error> {
    sqlx::query_as::<_, Education>(
        r#"
        INSERT INTO educations (freelancer_id, institution, degree, field_of_study, graduation_year)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(freelancer_id)
    .bind(institution)
    .bind(degree)
    .bind(field_of_study)
    .bind(graduation_year)
    .fetch_one(pool)
    .await
}

// Get all education details for a freelancer
pub async fn get_educations_for_freelancer(
    pool: &PgPool,
    freelancer_id: i32,
) -> Result<Vec<Education>, sqlx::Error> {
    sqlx::query_as::<_, Education>(
        "SELECT * FROM educations WHERE freelancer_id = $1",
    )
    .bind(freelancer_id)
    .fetch_all(pool)
    .await
}


// Add a client review for a freelancer
pub async fn add_client_review(
    pool: &PgPool,
    client_id: i32,
    freelancer_id: i32,
    feedback: String,
    rating: f64,
) -> Result<ClientReview, sqlx::Error> {
    sqlx::query_as::<_, ClientReview>(
        r#"
        INSERT INTO client_reviews (client_id, freelancer_id, feedback, rating, created_at)
        VALUES ($1, $2, $3, $4, NOW())
        RETURNING *
        "#,
    )
    .bind(client_id)
    .bind(freelancer_id)
    .bind(feedback)
    .bind(rating)
    .fetch_all()
    .await
}   



// Add a new review
pub async fn add_review(
    pool: &PgPool,
    input: ClientReview,
) -> Result<Review, sqlx::Error> {
    let overall_rating = (input.communication_rating + input.quality_rating + input.punctuality_rating) as f64 / 3.0;

    sqlx::query_as::<_, Review>(
        r#"
        INSERT INTO reviews (
            client_id, freelancer_id, communication_rating, quality_rating, punctuality_rating, feedback, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING *
        "#,
    )
    .bind(input.client_id)
    .bind(input.freelancer_id)
    .bind(input.communication_rating)
    .bind(input.quality_rating)
    .bind(input.punctuality_rating)
    .bind(&input.feedback)
    .fetch_one(pool)
    .await
}

// Get all reviews for a specific freelancer
pub async fn get_reviews_for_freelancer(
    pool: &PgPool,
    freelancer_id: i32,
) -> Result<Vec<Review>, sqlx::Error> {
    sqlx::query_as::<_, Review>("SELECT * FROM reviews WHERE freelancer_id = $1")
        .bind(freelancer_id)
        .fetch_all(pool)
        .await
}

// Get paginated reviews for a specific freelancer
pub async fn get_paginated_reviews_for_freelancer(
    pool: &PgPool,
    freelancer_id: i32,
    params: PaginationParams,
) -> Result<PaginatedReviews, sqlx::Error> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let reviews = sqlx::query_as::<_, Review>(
        r#"
        SELECT * FROM reviews
        WHERE freelancer_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(freelancer_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total_reviews: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reviews WHERE freelancer_id = $1")
        .bind(freelancer_id)
        .fetch_one(pool)
        .await?;

    Ok(PaginatedReviews {
        reviews,
        total_reviews,
        page,
        per_page,
    })
}

// Update a review
pub async fn update_review(
    pool: &PgPool,
    review_id: i32,
    updated_data: UpdatedReview,
) -> Result<Review, sqlx::Error> {
    sqlx::query_as::<_, Review>(
        r#"
        UPDATE reviews
        SET 
            communication_rating = COALESCE($2, communication_rating),
            quality_rating = COALESCE($3, quality_rating),
            punctuality_rating = COALESCE($4, punctuality_rating),
            feedback = COALESCE($5, feedback)
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(review_id)
    .bind(updated_data.communication_rating)
    .bind(updated_data.quality_rating)
    .bind(updated_data.punctuality_rating)
    .bind(updated_data.feedback)
    .fetch_one(pool)
    .await
}

// Delete a review
pub async fn delete_review(
    pool: &PgPool,
    review_id: i32,
) -> Result<u64, sqlx::Error> {
    sqlx::query("DELETE FROM reviews WHERE id = $1")
        .bind(review_id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}

// Get aggregate ratings for a freelancer
pub async fn get_aggregate_ratings_for_freelancer(
    pool: &PgPool,
    freelancer_id: i32,
) -> Result<AggregateRatings, sqlx::Error> {
    let ratings: (Option<f64>, Option<f64>, Option<f64>) = sqlx::query_as(
        r#"
        SELECT 
            AVG(communication_rating) AS communication_rating,
            AVG(quality_rating) AS quality_rating,
            AVG(punctuality_rating) AS punctuality_rating
        FROM reviews
        WHERE freelancer_id = $1
        "#,
    )
    .bind(freelancer_id)
    .fetch_one(pool)
    .await?;

    let communication_rating = ratings.0.unwrap_or(0.0);
    let quality_rating = ratings.1.unwrap_or(0.0);
    let punctuality_rating = ratings.2.unwrap_or(0.0);
    let overall_rating = (communication_rating + quality_rating + punctuality_rating) / 3.0;

    Ok(AggregateRatings {
        communication_rating,
        quality_rating,
        punctuality_rating,
        overall_rating,
    })
}

// ================== Workflow Queries ==================
pub async fn create_workflow(pool: &PgPool, workflow: &Workflow) -> Result<Workflow, Box<dyn StdError>> {
    let result = sqlx::query_as!(
        Workflow,
        r#"
        INSERT INTO workflows (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description
        "#,
        workflow.name,
        workflow.description
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(result)
}

pub async fn read_workflow(pool: &PgPool, workflow_id: i32) -> Result<Workflow, Box<dyn StdError>> {
    let result = sqlx::query_as!(
        Workflow,
        r#"
        SELECT id, name, description
        FROM workflows
        WHERE id = $1
        "#,
        workflow_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(result)
}

pub async fn update_workflow(pool: &PgPool, workflow_id: i32, updated_workflow: &Workflow) -> Result<Workflow, Box<dyn StdError>> {
    let result = sqlx::query_as!(
        Workflow,
        r#"
        UPDATE workflows
        SET name = $1, description = $2
        WHERE id = $3
        RETURNING id, name, description
        "#,
        updated_workflow.name,
        updated_workflow.description,
        workflow_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(result)
}

pub async fn delete_workflow(pool: &PgPool, workflow_id: i32) -> Result<(), Box<dyn StdError>> {
    sqlx::query!(
        r#"
        DELETE FROM workflows
        WHERE id = $1
        "#,
        workflow_id
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

    Ok(())
}

// ================== PROJECT Template Queries ==================

pub async fn create_template(conn: &PgPool, template: &ProjectTemplate) -> Result<ProjectTemplate, Error> {
    let query = r#"
        INSERT INTO project_templates (default_budget, default_category, default_description)
        VALUES ($1, $2, $3)
        RETURNING id, default_budget, default_category, default_description;
    "#;

    let row = sqlx::query_as::<_, ProjectTemplate>(query)
        .bind(&template.default_budget)
        .bind(&template.default_category)
        .bind(&template.default_description)
        .fetch_one(conn)
        .await?;

    Ok(row)
}

pub async fn apply_template(conn: &PgPool, template_id: i32, project: &mut Project) -> Result<(), Error> {
    let query = r#"
        SELECT default_budget, default_category, default_description
        FROM project_templates
        WHERE id = $1;
    "#;

    let template: ProjectTemplate = sqlx::query_as(query)
        .bind(template_id)
        .fetch_one(conn)
        .await?;

    project.budget = template.default_budget;
    project.category = template.default_category;
    project.description = template.default_description;

    Ok(())
}

pub async fn get_template(conn: &PgPool, template_id: i32) -> Result<ProjectTemplate, Error> {
    let query = r#"
        SELECT id, default_budget, default_category, default_description
        FROM project_templates
        WHERE id = $1;
    "#;

    let template = sqlx::query_as::<_, ProjectTemplate>(query)
        .bind(template_id)
        .fetch_one(conn)
        .await?;

    Ok(template)
}



pub async fn delete_template(conn: &PgPool, template_id: i32) -> Result<u64, Error> {
    let query = r#"
        DELETE FROM project_templates
        WHERE id = $1;
    "#;

    let rows_affected = sqlx::query(query)
        .bind(template_id)
        .execute(conn)
        .await?
        .rows_affected();

    Ok(rows_affected)
}


pub async fn update_template(conn: &PgPool, template_id: i32, updated_template: &ProjectTemplate) -> Result<ProjectTemplate, Error> {
    let query = r#"
        UPDATE project_templates
        SET default_budget = $1, default_category = $2, default_description = $3
        WHERE id = $4
        RETURNING id, default_budget, default_category, default_description;
    "#;

    let updated_row = sqlx::query_as::<_, ProjectTemplate>(query)
        .bind(&updated_template.default_budget)
        .bind(&updated_template.default_category)
        .bind(&updated_template.default_description)
        .bind(template_id)
        .fetch_one(conn)
        .await?;

    Ok(updated_row)
}
