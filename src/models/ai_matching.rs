
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMatchingEngine {
    pub skill_embeddings: HashMap<String, Vec<f32>>,
    pub freelancer_profiles: Vec<FreelancerEmbedding>,
    pub project_requirements: Vec<ProjectEmbedding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreelancerEmbedding {
    pub freelancer_id: Uuid,
    pub skills: Vec<String>,
    pub skill_levels: HashMap<String, f32>,
    pub experience_years: f32,
    pub avg_rating: f32,
    pub completion_rate: f32,
    pub timezone: String,
    pub hourly_rate: f64,
    pub availability: bool,
    pub past_project_types: Vec<String>,
    pub communication_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEmbedding {
    pub project_id: i32,
    pub required_skills: Vec<String>,
    pub skill_importance: HashMap<String, f32>,
    pub budget: f64,
    pub estimated_hours: i32,
    pub complexity_level: f32,
    pub timezone_preference: Option<String>,
    pub project_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MatchScore {
    pub freelancer_id: Uuid,
    pub project_id: i32,
    pub overall_score: f64,
    pub skill_match: f64,
    pub experience_match: f64,
    pub budget_fit: f64,
    pub success_probability: f64,
    pub reasons: Vec<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchRequest {
    pub freelancer_id: Option<Uuid>,
    pub project_id: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchExplanation {
    pub match_score: MatchScore,
    pub detailed_breakdown: HashMap<String, f64>,
    pub strengths: Vec<String>,
    pub potential_concerns: Vec<String>,
    pub recommendations: Vec<String>,
}

// Team Composition Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamCompositionSuggestion {
    pub suggested_members: Vec<SuggestedMember>,
    pub skill_coverage: f64,
    pub estimated_efficiency: f64,
    pub synergy_score: f64,
    pub cost_estimate: f64,
    pub timeline_estimate: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedMember {
    pub freelancer_id: Uuid,
    pub role: String,
    pub skills_contributed: Vec<String>,
    pub confidence: f64,
    pub alternative_candidates: Vec<Uuid>,
    pub estimated_hours: i32,
    pub hourly_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamCompositionRequest {
    pub project_id: i32,
    pub max_team_size: Option<i32>,
    pub budget_limit: Option<f64>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MatchHistory {
    pub id: i32,
    pub freelancer_id: Uuid,
    pub project_id: i32,
    pub match_score: f64,
    pub was_hired: bool,
    pub project_success: Option<bool>,
    pub created_at: NaiveDateTime,
}

// Dynamic Team Formation Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicTeamSuggestion {
    pub team_members: Vec<TeamMemberProfile>,
    pub synergy_score: f64,
    pub skill_coverage: f64,
    pub collaboration_score: f64,
    pub timezone_compatibility: f64,
    pub communication_compatibility: f64,
    pub estimated_success_rate: f64,
    pub team_dynamics: TeamDynamics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberProfile {
    pub freelancer_id: Uuid,
    pub role: String,
    pub skills: Vec<String>,
    pub availability_score: f64,
    pub timezone: String,
    pub communication_style: CommunicationStyle,
    pub past_team_success_rate: f64,
    pub collaboration_history: Vec<CollaborationHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Proactive,
    Responsive,
    Detailed,
    Concise,
    Collaborative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationHistory {
    pub partner_id: Uuid,
    pub project_count: i32,
    pub success_rate: f64,
    pub avg_rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamDynamics {
    pub leadership_score: f64,
    pub diversity_score: f64,
    pub experience_balance: f64,
    pub potential_conflicts: Vec<String>,
    pub strengths: Vec<String>,
}

// Skill Synergy Models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSynergyAnalysis {
    pub synergy_score: f64,
    pub complementary_skills: Vec<SkillPair>,
    pub skill_gaps: Vec<SkillGap>,
    pub skill_overlaps: Vec<SkillOverlap>,
    pub optimization_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPair {
    pub skill_a: String,
    pub skill_b: String,
    pub synergy_level: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGap {
    pub missing_skill: String,
    pub importance: f64,
    pub impact_on_project: String,
    pub suggested_candidates: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillOverlap {
    pub skill: String,
    pub redundancy_level: f64,
    pub team_members_with_skill: Vec<Uuid>,
    pub optimization_suggestion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicTeamRequest {
    pub project_id: i32,
    pub required_skills: Vec<String>,
    pub max_team_size: Option<i32>,
    pub budget_limit: Option<f64>,
    pub timezone_preference: Option<String>,
    pub prioritize_past_collaborations: bool,
}
