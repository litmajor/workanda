
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalAssistant {
    pub suggested_structure: Vec<Section>,
    pub key_points: Vec<String>,
    pub client_preferences: ClientProfile,
    pub estimated_win_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub suggested_content: String,
    pub importance: f64,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientProfile {
    pub communication_style: String,
    pub budget_sensitivity: f64,
    pub timeline_flexibility: f64,
    pub preferred_proposal_length: String,
    pub key_decision_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalAnalysisRequest {
    pub job_id: i32,
    pub freelancer_id: Uuid,
    pub draft_proposal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalAnalysisResponse {
    pub suggestions: Vec<String>,
    pub estimated_win_rate: f64,
    pub improvements: ProposalImprovements,
    pub assistant: ProposalAssistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalImprovements {
    pub structure_score: f64,
    pub relevance_score: f64,
    pub clarity_score: f64,
    pub professionalism_score: f64,
    pub missing_elements: Vec<String>,
    pub strengths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCategorization {
    pub primary_category: String,
    pub subcategories: Vec<String>,
    pub required_skills: Vec<String>,
    pub optional_skills: Vec<String>,
    pub project_type: ProjectType,
    pub complexity_level: ComplexityLevel,
    pub suggested_budget_range: (f64, f64),
    pub team_suitable: bool,
    pub estimated_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    WebDevelopment,
    MobileDevelopment,
    DataScience,
    Design,
    Writing,
    Marketing,
    Consulting,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartSearchRequest {
    pub query: String,
    pub search_type: SearchType,
    pub limit: Option<i32>,
    pub filters: Option<SearchFilters>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    Projects,
    Freelancers,
    Skills,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub budget_range: Option<(f64, f64)>,
    pub skills: Option<Vec<String>>,
    pub location: Option<String>,
    pub availability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartSearchResponse {
    pub results: Vec<SearchResult>,
    pub intent_analysis: IntentAnalysis,
    pub recommendations: Vec<String>,
    pub trending_skills: Vec<TrendingSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub result_type: String,
    pub title: String,
    pub description: String,
    pub relevance_score: f64,
    pub match_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAnalysis {
    pub detected_intent: String,
    pub key_terms: Vec<String>,
    pub suggested_refinements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingSkill {
    pub skill_name: String,
    pub demand_score: f64,
    pub growth_rate: f64,
    pub avg_rate: f64,
}
