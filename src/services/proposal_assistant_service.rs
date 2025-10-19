
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;
use crate::models::proposal_assistant::*;

pub struct ProposalAssistantService {
    pool: PgPool,
}

impl ProposalAssistantService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn analyze_proposal(
        &self,
        request: ProposalAnalysisRequest,
    ) -> Result<ProposalAnalysisResponse, String> {
        // Fetch job details
        let job = self.fetch_job_details(request.job_id).await?;
        
        // Fetch freelancer profile
        let freelancer = self.fetch_freelancer_profile(request.freelancer_id).await?;
        
        // Fetch client preferences
        let client_profile = self.analyze_client_preferences(job.client_id).await?;
        
        // Analyze job requirements
        let job_requirements = self.extract_job_requirements(&job.description).await?;
        
        // Generate proposal structure
        let suggested_structure = self.generate_proposal_structure(&job_requirements);
        
        // Generate key points
        let key_points = self.generate_key_points(&job_requirements, &freelancer);
        
        // Analyze draft proposal if provided
        let improvements = if let Some(draft) = &request.draft_proposal {
            self.analyze_draft_proposal(draft, &job_requirements, &client_profile).await?
        } else {
            ProposalImprovements {
                structure_score: 0.0,
                relevance_score: 0.0,
                clarity_score: 0.0,
                professionalism_score: 0.0,
                missing_elements: vec![],
                strengths: vec![],
            }
        };
        
        // Calculate win rate
        let estimated_win_rate = self.calculate_win_rate(
            &freelancer,
            &job_requirements,
            &improvements,
        );
        
        // Generate suggestions
        let suggestions = self.generate_suggestions(
            &job_requirements,
            &freelancer,
            &client_profile,
            &improvements,
        );
        
        Ok(ProposalAnalysisResponse {
            suggestions,
            estimated_win_rate,
            improvements,
            assistant: ProposalAssistant {
                suggested_structure,
                key_points,
                client_preferences: client_profile,
                estimated_win_rate,
            },
        })
    }

    pub async fn categorize_job(&self, job_id: i32) -> Result<JobCategorization, String> {
        let job = self.fetch_job_details(job_id).await?;
        
        // Extract skills from description
        let skills = self.extract_skills(&job.description);
        
        // Determine project type
        let project_type = self.classify_project_type(&job.description);
        
        // Calculate complexity
        let complexity_level = self.assess_complexity(&job.description, &skills);
        
        // Suggest budget range
        let suggested_budget_range = self.suggest_budget_range(&project_type, &complexity_level);
        
        // Determine if team suitable
        let team_suitable = self.is_team_suitable(&complexity_level, &skills);
        
        Ok(JobCategorization {
            primary_category: self.determine_primary_category(&project_type),
            subcategories: self.determine_subcategories(&skills),
            required_skills: skills.required,
            optional_skills: skills.optional,
            project_type,
            complexity_level,
            suggested_budget_range,
            team_suitable,
            estimated_duration: self.estimate_duration(&complexity_level),
        })
    }

    pub async fn smart_search(
        &self,
        request: SmartSearchRequest,
    ) -> Result<SmartSearchResponse, String> {
        // Analyze search intent
        let intent_analysis = self.analyze_search_intent(&request.query);
        
        // Perform semantic search
        let results = self.semantic_search(&request).await?;
        
        // Generate recommendations
        let recommendations = self.generate_search_recommendations(&intent_analysis, &results);
        
        // Get trending skills
        let trending_skills = self.get_trending_skills().await?;
        
        Ok(SmartSearchResponse {
            results,
            intent_analysis,
            recommendations,
            trending_skills,
        })
    }

    // Helper methods
    async fn fetch_job_details(&self, job_id: i32) -> Result<JobDetails, String> {
        let job = sqlx::query!(
            "SELECT id, title, description, budget, client_id FROM jobs WHERE id = $1",
            job_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to fetch job: {}", e))?;
        
        Ok(JobDetails {
            id: job.id,
            title: job.title,
            description: job.description.unwrap_or_default(),
            budget: job.budget,
            client_id: job.client_id,
        })
    }

    async fn fetch_freelancer_profile(&self, freelancer_id: Uuid) -> Result<FreelancerProfile, String> {
        let profile = sqlx::query!(
            "SELECT specializations, hourly_rate, category FROM freelancer_accounts WHERE user_id = $1",
            freelancer_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to fetch freelancer: {}", e))?;
        
        Ok(FreelancerProfile {
            id: freelancer_id,
            skills: profile.specializations,
            hourly_rate: profile.hourly_rate,
            category: profile.category,
        })
    }

    async fn analyze_client_preferences(&self, client_id: u32) -> Result<ClientProfile, String> {
        // Analyze past job postings and accepted proposals
        Ok(ClientProfile {
            communication_style: "professional".to_string(),
            budget_sensitivity: 0.7,
            timeline_flexibility: 0.6,
            preferred_proposal_length: "detailed".to_string(),
            key_decision_factors: vec![
                "expertise".to_string(),
                "timeline".to_string(),
                "budget".to_string(),
            ],
        })
    }

    async fn extract_job_requirements(&self, description: &str) -> Result<JobRequirements, String> {
        // NLP to extract requirements
        let keywords = self.extract_keywords(description);
        
        Ok(JobRequirements {
            skills: keywords.clone(),
            mentioned_technologies: keywords.clone(),
            deliverables: vec![],
            timeline_mentions: 0,
        })
    }

    fn generate_proposal_structure(&self, requirements: &JobRequirements) -> Vec<Section> {
        vec![
            Section {
                title: "Introduction".to_string(),
                suggested_content: "Brief introduction highlighting relevant experience".to_string(),
                importance: 0.9,
                examples: vec!["I'm excited about this opportunity...".to_string()],
            },
            Section {
                title: "Understanding of Requirements".to_string(),
                suggested_content: "Demonstrate you understand the project needs".to_string(),
                importance: 1.0,
                examples: vec!["Based on your requirements, I understand you need...".to_string()],
            },
            Section {
                title: "Proposed Solution".to_string(),
                suggested_content: "Outline your approach to the project".to_string(),
                importance: 1.0,
                examples: vec!["My approach will be to...".to_string()],
            },
            Section {
                title: "Timeline & Deliverables".to_string(),
                suggested_content: "Provide specific timeline and deliverables".to_string(),
                importance: 0.95,
                examples: vec!["Week 1: ..., Week 2: ...".to_string()],
            },
            Section {
                title: "Budget Breakdown".to_string(),
                suggested_content: "Detailed cost breakdown".to_string(),
                importance: 0.85,
                examples: vec!["Development: $X, Testing: $Y".to_string()],
            },
        ]
    }

    fn generate_key_points(&self, requirements: &JobRequirements, freelancer: &FreelancerProfile) -> Vec<String> {
        let mut points = vec![];
        
        for skill in &requirements.skills {
            if freelancer.skills.contains(skill) {
                points.push(format!("Emphasize your {} expertise", skill));
            }
        }
        
        points.push("Add specific timeline for deliverables".to_string());
        points.push("Include detailed cost breakdown".to_string());
        points.push("Showcase relevant portfolio examples".to_string());
        
        points
    }

    async fn analyze_draft_proposal(
        &self,
        draft: &str,
        requirements: &JobRequirements,
        client_profile: &ClientProfile,
    ) -> Result<ProposalImprovements, String> {
        let word_count = draft.split_whitespace().count();
        
        let structure_score = if word_count > 200 && word_count < 800 { 0.9 } else { 0.6 };
        let relevance_score = self.calculate_relevance_score(draft, requirements);
        let clarity_score = 0.8; // Simplified
        let professionalism_score = 0.85; // Simplified
        
        let missing_elements = self.identify_missing_elements(draft);
        let strengths = self.identify_strengths(draft);
        
        Ok(ProposalImprovements {
            structure_score,
            relevance_score,
            clarity_score,
            professionalism_score,
            missing_elements,
            strengths,
        })
    }

    fn calculate_win_rate(
        &self,
        freelancer: &FreelancerProfile,
        requirements: &JobRequirements,
        improvements: &ProposalImprovements,
    ) -> f64 {
        let skill_match = self.calculate_skill_match(&freelancer.skills, &requirements.skills);
        let proposal_quality = (improvements.structure_score + improvements.relevance_score + 
                               improvements.clarity_score + improvements.professionalism_score) / 4.0;
        
        (skill_match * 0.6 + proposal_quality * 0.4).min(1.0)
    }

    fn generate_suggestions(
        &self,
        requirements: &JobRequirements,
        freelancer: &FreelancerProfile,
        client_profile: &ClientProfile,
        improvements: &ProposalImprovements,
    ) -> Vec<String> {
        let mut suggestions = vec![];
        
        for skill in &requirements.skills {
            if freelancer.skills.contains(skill) {
                suggestions.push(format!("Emphasize your {} expertise - it's mentioned in job description", skill));
            }
        }
        
        if improvements.structure_score < 0.7 {
            suggestions.push("Improve proposal structure with clear sections".to_string());
        }
        
        if client_profile.budget_sensitivity > 0.7 {
            suggestions.push("Client prefers detailed cost breakdown".to_string());
        }
        
        suggestions.push("Add specific timeline for deliverables".to_string());
        
        suggestions
    }

    fn extract_skills(&self, description: &str) -> ExtractedSkills {
        let common_skills = vec![
            "react", "vue", "angular", "python", "rust", "javascript",
            "typescript", "node", "docker", "kubernetes", "aws", "design",
        ];
        
        let description_lower = description.to_lowercase();
        let mut required = vec![];
        let mut optional = vec![];
        
        for skill in common_skills {
            if description_lower.contains(skill) {
                if description_lower.contains(&format!("required {}", skill)) ||
                   description_lower.contains(&format!("{} required", skill)) {
                    required.push(skill.to_string());
                } else {
                    optional.push(skill.to_string());
                }
            }
        }
        
        ExtractedSkills { required, optional }
    }

    fn classify_project_type(&self, description: &str) -> ProjectType {
        let description_lower = description.to_lowercase();
        
        if description_lower.contains("web") || description_lower.contains("website") {
            ProjectType::WebDevelopment
        } else if description_lower.contains("mobile") || description_lower.contains("app") {
            ProjectType::MobileDevelopment
        } else if description_lower.contains("data") || description_lower.contains("analytics") {
            ProjectType::DataScience
        } else if description_lower.contains("design") || description_lower.contains("ui") {
            ProjectType::Design
        } else {
            ProjectType::Other("General".to_string())
        }
    }

    fn assess_complexity(&self, description: &str, skills: &ExtractedSkills) -> ComplexityLevel {
        let total_skills = skills.required.len() + skills.optional.len();
        
        if total_skills > 10 || description.len() > 2000 {
            ComplexityLevel::VeryComplex
        } else if total_skills > 5 || description.len() > 1000 {
            ComplexityLevel::Complex
        } else if total_skills > 2 {
            ComplexityLevel::Moderate
        } else {
            ComplexityLevel::Simple
        }
    }

    fn suggest_budget_range(&self, project_type: &ProjectType, complexity: &ComplexityLevel) -> (f64, f64) {
        let base = match project_type {
            ProjectType::WebDevelopment => (1000.0, 5000.0),
            ProjectType::MobileDevelopment => (2000.0, 10000.0),
            ProjectType::DataScience => (1500.0, 8000.0),
            ProjectType::Design => (500.0, 3000.0),
            _ => (500.0, 5000.0),
        };
        
        let multiplier = match complexity {
            ComplexityLevel::Simple => 0.5,
            ComplexityLevel::Moderate => 1.0,
            ComplexityLevel::Complex => 1.5,
            ComplexityLevel::VeryComplex => 2.5,
        };
        
        (base.0 * multiplier, base.1 * multiplier)
    }

    fn is_team_suitable(&self, complexity: &ComplexityLevel, skills: &ExtractedSkills) -> bool {
        matches!(complexity, ComplexityLevel::Complex | ComplexityLevel::VeryComplex) ||
        skills.required.len() > 5
    }

    fn determine_primary_category(&self, project_type: &ProjectType) -> String {
        match project_type {
            ProjectType::WebDevelopment => "Web Development".to_string(),
            ProjectType::MobileDevelopment => "Mobile Development".to_string(),
            ProjectType::DataScience => "Data Science".to_string(),
            ProjectType::Design => "Design".to_string(),
            ProjectType::Other(s) => s.clone(),
            _ => "Other".to_string(),
        }
    }

    fn determine_subcategories(&self, skills: &ExtractedSkills) -> Vec<String> {
        skills.required.clone()
    }

    fn estimate_duration(&self, complexity: &ComplexityLevel) -> String {
        match complexity {
            ComplexityLevel::Simple => "1-2 weeks".to_string(),
            ComplexityLevel::Moderate => "2-4 weeks".to_string(),
            ComplexityLevel::Complex => "1-3 months".to_string(),
            ComplexityLevel::VeryComplex => "3+ months".to_string(),
        }
    }

    fn analyze_search_intent(&self, query: &str) -> IntentAnalysis {
        let key_terms = self.extract_keywords(query);
        
        IntentAnalysis {
            detected_intent: "finding projects".to_string(),
            key_terms,
            suggested_refinements: vec!["Add budget range".to_string(), "Specify skills".to_string()],
        }
    }

    async fn semantic_search(&self, request: &SmartSearchRequest) -> Result<Vec<SearchResult>, String> {
        // Simplified semantic search
        Ok(vec![])
    }

    fn generate_search_recommendations(&self, intent: &IntentAnalysis, results: &[SearchResult]) -> Vec<String> {
        vec![
            "Try searching for related skills".to_string(),
            "Expand your search radius".to_string(),
        ]
    }

    async fn get_trending_skills(&self) -> Result<Vec<TrendingSkill>, String> {
        Ok(vec![
            TrendingSkill {
                skill_name: "React".to_string(),
                demand_score: 0.95,
                growth_rate: 0.15,
                avg_rate: 75.0,
            },
            TrendingSkill {
                skill_name: "Rust".to_string(),
                demand_score: 0.85,
                growth_rate: 0.25,
                avg_rate: 90.0,
            },
        ])
    }

    fn extract_keywords(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .filter(|w| w.len() > 3)
            .take(10)
            .map(|s| s.to_string())
            .collect()
    }

    fn calculate_relevance_score(&self, draft: &str, requirements: &JobRequirements) -> f64 {
        let mut matches = 0;
        let draft_lower = draft.to_lowercase();
        
        for skill in &requirements.skills {
            if draft_lower.contains(&skill.to_lowercase()) {
                matches += 1;
            }
        }
        
        if requirements.skills.is_empty() {
            0.5
        } else {
            matches as f64 / requirements.skills.len() as f64
        }
    }

    fn identify_missing_elements(&self, draft: &str) -> Vec<String> {
        let mut missing = vec![];
        
        if !draft.to_lowercase().contains("timeline") {
            missing.push("Timeline section".to_string());
        }
        if !draft.to_lowercase().contains("budget") && !draft.contains("$") {
            missing.push("Budget breakdown".to_string());
        }
        
        missing
    }

    fn identify_strengths(&self, draft: &str) -> Vec<String> {
        let mut strengths = vec![];
        
        if draft.len() > 300 {
            strengths.push("Good length and detail".to_string());
        }
        if draft.contains("experience") {
            strengths.push("Highlights experience".to_string());
        }
        
        strengths
    }

    fn calculate_skill_match(&self, freelancer_skills: &[String], required_skills: &[String]) -> f64 {
        if required_skills.is_empty() {
            return 0.5;
        }
        
        let matches = freelancer_skills.iter()
            .filter(|s| required_skills.contains(s))
            .count();
        
        matches as f64 / required_skills.len() as f64
    }
}

// Helper structs
struct JobDetails {
    id: i32,
    title: String,
    description: String,
    budget: Option<f64>,
    client_id: u32,
}

struct FreelancerProfile {
    id: Uuid,
    skills: Vec<String>,
    hourly_rate: Option<f64>,
    category: String,
}

struct JobRequirements {
    skills: Vec<String>,
    mentioned_technologies: Vec<String>,
    deliverables: Vec<String>,
    timeline_mentions: i32,
}

struct ExtractedSkills {
    required: Vec<String>,
    optional: Vec<String>,
}
