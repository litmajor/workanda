
use crate::models::ai_matching::*;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;

pub struct AIMatchingService {
    pool: PgPool,
}

impl AIMatchingService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculate skill match score between freelancer and project
    fn calculate_skill_match(
        freelancer: &FreelancerEmbedding,
        project: &ProjectEmbedding,
    ) -> (f64, Vec<String>) {
        let mut matched_skills = 0;
        let mut total_importance = 0.0;
        let mut weighted_match = 0.0;
        let mut reasons = Vec::new();

        for required_skill in &project.required_skills {
            let importance = project.skill_importance.get(required_skill).unwrap_or(&1.0);
            total_importance += importance;

            if freelancer.skills.contains(required_skill) {
                let skill_level = freelancer.skill_levels.get(required_skill).unwrap_or(&0.5);
                weighted_match += importance * skill_level;
                matched_skills += 1;
                reasons.push(format!("Strong match in {}", required_skill));
            }
        }

        let skill_coverage = matched_skills as f64 / project.required_skills.len() as f64;
        let weighted_score = if total_importance > 0.0 {
            weighted_match / total_importance
        } else {
            skill_coverage
        };

        let final_score = (skill_coverage * 0.4) + (weighted_score * 0.6);
        
        if matched_skills < project.required_skills.len() {
            reasons.push(format!(
                "Missing {} required skills",
                project.required_skills.len() - matched_skills
            ));
        }

        (final_score, reasons)
    }

    /// Calculate experience match based on project complexity
    fn calculate_experience_match(
        freelancer: &FreelancerEmbedding,
        project: &ProjectEmbedding,
    ) -> f64 {
        let experience_score = (freelancer.experience_years / 10.0).min(1.0);
        let complexity_fit = 1.0 - (project.complexity_level - experience_score).abs();
        
        // Weight by past success
        let success_weight = freelancer.completion_rate * freelancer.avg_rating / 5.0;
        
        (complexity_fit * 0.6) + (success_weight * 0.4)
    }

    /// Calculate budget fit
    fn calculate_budget_fit(
        freelancer: &FreelancerEmbedding,
        project: &ProjectEmbedding,
    ) -> f64 {
        let estimated_cost = freelancer.hourly_rate * project.estimated_hours as f64;
        let budget_ratio = estimated_cost / project.budget;

        if budget_ratio <= 0.8 {
            1.0 // Under budget
        } else if budget_ratio <= 1.0 {
            0.9 // Within budget
        } else if budget_ratio <= 1.2 {
            0.7 // Slightly over budget
        } else {
            0.3 // Significantly over budget
        }
    }

    /// Calculate overall success probability
    fn calculate_success_probability(
        skill_match: f64,
        experience_match: f64,
        budget_fit: f64,
        freelancer: &FreelancerEmbedding,
    ) -> f64 {
        let base_probability = (skill_match * 0.4) + (experience_match * 0.3) + (budget_fit * 0.2);
        let track_record = freelancer.completion_rate * 0.1;
        
        (base_probability + track_record).min(1.0)
    }

    /// Match freelancer to best projects
    pub async fn match_freelancer_to_projects(
        &self,
        freelancer_id: Uuid,
        limit: i32,
    ) -> Result<Vec<MatchScore>, sqlx::Error> {
        // Get freelancer data
        let freelancer = self.get_freelancer_embedding(freelancer_id).await?;
        
        // Get available projects
        let projects = self.get_available_projects().await?;
        
        let mut matches = Vec::new();

        for project in projects {
            let (skill_match, mut reasons) = Self::calculate_skill_match(&freelancer, &project);
            let experience_match = Self::calculate_experience_match(&freelancer, &project);
            let budget_fit = Self::calculate_budget_fit(&freelancer, &project);
            let success_probability = Self::calculate_success_probability(
                skill_match,
                experience_match,
                budget_fit,
                &freelancer,
            );

            let overall_score = (skill_match * 0.35)
                + (experience_match * 0.25)
                + (budget_fit * 0.20)
                + (success_probability * 0.20);

            if experience_match > 0.7 {
                reasons.push("Experience level matches project complexity".to_string());
            }
            if budget_fit > 0.8 {
                reasons.push("Rate fits within project budget".to_string());
            }

            matches.push(MatchScore {
                freelancer_id,
                project_id: project.project_id,
                overall_score,
                skill_match,
                experience_match,
                budget_fit,
                success_probability,
                reasons,
                created_at: chrono::Utc::now().naive_utc(),
            });
        }

        matches.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());
        matches.truncate(limit as usize);

        Ok(matches)
    }

    /// Match project to best freelancers
    pub async fn match_project_to_freelancers(
        &self,
        project_id: i32,
        limit: i32,
    ) -> Result<Vec<MatchScore>, sqlx::Error> {
        let project = self.get_project_embedding(project_id).await?;
        let freelancers = self.get_available_freelancers().await?;

        let mut matches = Vec::new();

        for freelancer in freelancers {
            let (skill_match, mut reasons) = Self::calculate_skill_match(&freelancer, &project);
            let experience_match = Self::calculate_experience_match(&freelancer, &project);
            let budget_fit = Self::calculate_budget_fit(&freelancer, &project);
            let success_probability = Self::calculate_success_probability(
                skill_match,
                experience_match,
                budget_fit,
                &freelancer,
            );

            let overall_score = (skill_match * 0.35)
                + (experience_match * 0.25)
                + (budget_fit * 0.20)
                + (success_probability * 0.20);

            if freelancer.availability {
                reasons.push("Currently available".to_string());
            }
            if freelancer.avg_rating >= 4.5 {
                reasons.push("Highly rated freelancer".to_string());
            }

            matches.push(MatchScore {
                freelancer_id: freelancer.freelancer_id,
                project_id,
                overall_score,
                skill_match,
                experience_match,
                budget_fit,
                success_probability,
                reasons,
                created_at: chrono::Utc::now().naive_utc(),
            });
        }

        matches.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());
        matches.truncate(limit as usize);

        Ok(matches)
    }

    /// Suggest optimal team composition for a project
    pub async fn suggest_team_composition(
        &self,
        project_id: i32,
        max_team_size: i32,
        budget_limit: Option<f64>,
    ) -> Result<TeamCompositionSuggestion, sqlx::Error> {
        let project = self.get_project_embedding(project_id).await?;
        let freelancers = self.get_available_freelancers().await?;

        // Group freelancers by their strongest skills
        let mut skill_specialists: HashMap<String, Vec<FreelancerEmbedding>> = HashMap::new();
        
        for freelancer in freelancers {
            for skill in &freelancer.skills {
                skill_specialists
                    .entry(skill.clone())
                    .or_insert_with(Vec::new)
                    .push(freelancer.clone());
            }
        }

        let mut suggested_members = Vec::new();
        let mut covered_skills = Vec::new();
        let mut total_cost = 0.0;

        // Select best freelancer for each required skill
        for required_skill in &project.required_skills {
            if let Some(specialists) = skill_specialists.get(required_skill) {
                let mut best_candidates: Vec<_> = specialists
                    .iter()
                    .filter(|f| !suggested_members.iter().any(|m: &SuggestedMember| m.freelancer_id == f.freelancer_id))
                    .collect();

                best_candidates.sort_by(|a, b| {
                    let score_a = a.skill_levels.get(required_skill).unwrap_or(&0.0);
                    let score_b = b.skill_levels.get(required_skill).unwrap_or(&0.0);
                    score_b.partial_cmp(score_a).unwrap()
                });

                if let Some(best) = best_candidates.first() {
                    let alternatives: Vec<Uuid> = best_candidates
                        .iter()
                        .skip(1)
                        .take(3)
                        .map(|f| f.freelancer_id)
                        .collect();

                    let estimated_hours = (project.estimated_hours as f64 / project.required_skills.len() as f64) as i32;
                    let member_cost = best.hourly_rate * estimated_hours as f64;

                    if let Some(limit) = budget_limit {
                        if total_cost + member_cost > limit {
                            continue;
                        }
                    }

                    total_cost += member_cost;
                    covered_skills.push(required_skill.clone());

                    suggested_members.push(SuggestedMember {
                        freelancer_id: best.freelancer_id,
                        role: required_skill.clone(),
                        skills_contributed: vec![required_skill.clone()],
                        confidence: *best.skill_levels.get(required_skill).unwrap_or(&0.5),
                        alternative_candidates: alternatives,
                        estimated_hours,
                        hourly_rate: best.hourly_rate,
                    });

                    if suggested_members.len() >= max_team_size as usize {
                        break;
                    }
                }
            }
        }

        let skill_coverage = covered_skills.len() as f64 / project.required_skills.len() as f64;
        let estimated_efficiency = suggested_members.iter().map(|m| m.confidence).sum::<f64>() / suggested_members.len() as f64;
        let synergy_score = 0.75; // Placeholder for future synergy calculation

        Ok(TeamCompositionSuggestion {
            suggested_members,
            skill_coverage,
            estimated_efficiency,
            synergy_score,
            cost_estimate: total_cost,
            timeline_estimate: project.estimated_hours,
        })
    }

    // Helper methods to fetch data
    async fn get_freelancer_embedding(&self, freelancer_id: Uuid) -> Result<FreelancerEmbedding, sqlx::Error> {
        let freelancer = sqlx::query!(
            r#"
            SELECT 
                fa.id,
                fa.hourly_rate,
                fa.specializations,
                fa.availability,
                fa.location,
                COALESCE(AVG(r.communication_rating + r.quality_rating + r.punctuality_rating) / 3.0, 0) as avg_rating,
                COUNT(DISTINCT c.id) as completed_projects
            FROM freelancer_accounts fa
            LEFT JOIN reviews r ON r.freelancer_id = fa.id
            LEFT JOIN contracts c ON c.freelancer_id = fa.id AND c.status = 'completed'
            WHERE fa.id = $1
            GROUP BY fa.id, fa.hourly_rate, fa.specializations, fa.availability, fa.location
            "#,
            freelancer_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(FreelancerEmbedding {
            freelancer_id,
            skills: freelancer.specializations.unwrap_or_default(),
            skill_levels: HashMap::new(), // Populate from skills table
            experience_years: 2.0, // Calculate from join date or past projects
            avg_rating: freelancer.avg_rating.unwrap_or(0.0) as f32,
            completion_rate: 0.85,
            timezone: freelancer.location.unwrap_or_default(),
            hourly_rate: freelancer.hourly_rate.unwrap_or(0.0),
            availability: true,
            past_project_types: vec![],
            communication_score: 0.8,
        })
    }

    async fn get_project_embedding(&self, project_id: i32) -> Result<ProjectEmbedding, sqlx::Error> {
        let project = sqlx::query!(
            r#"
            SELECT 
                id,
                title,
                description,
                budget,
                status
            FROM jobs
            WHERE id = $1
            "#,
            project_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ProjectEmbedding {
            project_id,
            required_skills: vec![], // Parse from description or separate table
            skill_importance: HashMap::new(),
            budget: project.budget.unwrap_or(0.0),
            estimated_hours: 40,
            complexity_level: 0.6,
            timezone_preference: None,
            project_type: "web_development".to_string(),
        })
    }

    async fn get_available_projects(&self) -> Result<Vec<ProjectEmbedding>, sqlx::Error> {
        // Fetch open projects and convert to embeddings
        Ok(vec![])
    }

    async fn get_available_freelancers(&self) -> Result<Vec<FreelancerEmbedding>, sqlx::Error> {
        // Fetch available freelancers and convert to embeddings
        Ok(vec![])
    }
}
