use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewSkill {
    pub name: String,
    pub description: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct FreelancerSkill {
    pub id: i32,
    pub freelancer_id: Uuid,
    pub skill_id: i32,
    pub proficiency_level: ProficiencyLevel,
}

#[derive(Serialize, Deserialize)]
pub struct NewFreelancerSkill {
    pub freelancer_id: Uuid,
    pub skill_id: i32,
    pub proficiency_level: ProficiencyLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ProficiencyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

pub async fn get_skills_for_freelancer(
    pool: &PgPool,
    freelancer_id: Uuid,
) -> Result<Vec<FreelancerSkill>, sqlx::Error> {
    sqlx::query_as::<_, FreelancerSkill>(
        "SELECT * FROM freelancer_skills WHERE freelancer_id = $1",
    )
    .bind(freelancer_id)
    .fetch_all(pool)
    .await
}
