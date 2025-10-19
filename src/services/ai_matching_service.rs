
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

    /// Dynamic Team Formation with AI-powered matching
    pub async fn form_dynamic_team(
        &self,
        project_id: i32,
        required_skills: Vec<String>,
        max_team_size: i32,
        budget_limit: Option<f64>,
        timezone_preference: Option<String>,
        prioritize_past_collaborations: bool,
    ) -> Result<DynamicTeamSuggestion, sqlx::Error> {
        let freelancers = self.get_available_freelancers().await?;
        let project = self.get_project_embedding(project_id).await?;

        // Calculate collaboration scores for all freelancer pairs
        let collaboration_matrix = self.build_collaboration_matrix(&freelancers).await?;

        // Find optimal team composition
        let mut best_team = Vec::new();
        let mut best_score = 0.0;

        // Use greedy algorithm with backtracking for team selection
        for seed_freelancer in &freelancers {
            let mut current_team = vec![seed_freelancer.clone()];
            let mut covered_skills = seed_freelancer.skills.clone();

            while current_team.len() < max_team_size as usize {
                let next_member = self.select_next_team_member(
                    &current_team,
                    &freelancers,
                    &required_skills,
                    &covered_skills,
                    &collaboration_matrix,
                    timezone_preference.as_deref(),
                )?;

                if let Some(member) = next_member {
                    covered_skills.extend(member.skills.clone());
                    current_team.push(member);
                } else {
                    break;
                }
            }

            let team_score = self.calculate_team_score(
                &current_team,
                &required_skills,
                &collaboration_matrix,
                timezone_preference.as_deref(),
            );

            if team_score > best_score {
                best_score = team_score;
                best_team = current_team;
            }
        }

        // Build team member profiles
        let team_members: Vec<TeamMemberProfile> = best_team
            .iter()
            .map(|f| TeamMemberProfile {
                freelancer_id: f.freelancer_id,
                role: self.determine_role(&f.skills, &required_skills),
                skills: f.skills.clone(),
                availability_score: if f.availability { 1.0 } else { 0.5 },
                timezone: f.timezone.clone(),
                communication_style: CommunicationStyle::Collaborative, // Fetch from history
                past_team_success_rate: f.completion_rate as f64,
                collaboration_history: vec![],
            })
            .collect();

        // Calculate synergy metrics
        let synergy_analysis = self.analyze_skill_synergy(&best_team, &required_skills);
        let skill_coverage = synergy_analysis.synergy_score;
        
        let collaboration_score = self.calculate_collaboration_score(&best_team, &collaboration_matrix);
        let timezone_compatibility = self.calculate_timezone_compatibility(&best_team, timezone_preference.as_deref());
        let communication_compatibility = 0.85; // Calculate from communication styles

        let team_dynamics = self.analyze_team_dynamics(&best_team, &required_skills);

        Ok(DynamicTeamSuggestion {
            team_members,
            synergy_score: synergy_analysis.synergy_score,
            skill_coverage,
            collaboration_score,
            timezone_compatibility,
            communication_compatibility,
            estimated_success_rate: (synergy_analysis.synergy_score * 0.3 
                + collaboration_score * 0.3 
                + skill_coverage * 0.4),
            team_dynamics,
        })
    }

    /// Analyze skill synergy within a team
    pub fn analyze_skill_synergy(
        &self,
        team: &[FreelancerEmbedding],
        required_skills: &[String],
    ) -> SkillSynergyAnalysis {
        let mut all_team_skills: Vec<String> = team
            .iter()
            .flat_map(|f| f.skills.clone())
            .collect();
        all_team_skills.sort();
        all_team_skills.dedup();

        // Find complementary skills
        let complementary_skills = self.find_complementary_skills(&all_team_skills);

        // Identify skill gaps
        let skill_gaps: Vec<SkillGap> = required_skills
            .iter()
            .filter(|s| !all_team_skills.contains(s))
            .map(|s| SkillGap {
                missing_skill: s.clone(),
                importance: 0.8,
                impact_on_project: format!("Critical skill {} is missing", s),
                suggested_candidates: vec![],
            })
            .collect();

        // Find skill overlaps
        let mut skill_overlaps = Vec::new();
        for skill in &all_team_skills {
            let members_with_skill: Vec<Uuid> = team
                .iter()
                .filter(|f| f.skills.contains(skill))
                .map(|f| f.freelancer_id)
                .collect();

            if members_with_skill.len() > 1 {
                let redundancy = (members_with_skill.len() - 1) as f64 / team.len() as f64;
                skill_overlaps.push(SkillOverlap {
                    skill: skill.clone(),
                    redundancy_level: redundancy,
                    team_members_with_skill: members_with_skill,
                    optimization_suggestion: if redundancy > 0.5 {
                        format!("Consider replacing one member with {} skill", skill)
                    } else {
                        "Acceptable overlap for redundancy".to_string()
                    },
                });
            }
        }

        let coverage = all_team_skills.iter().filter(|s| required_skills.contains(s)).count() as f64 
            / required_skills.len() as f64;

        let synergy_score = coverage * (1.0 - (skill_gaps.len() as f64 * 0.1))
            * (1.0 + complementary_skills.len() as f64 * 0.05);

        let optimization_suggestions = self.generate_optimization_suggestions(&skill_gaps, &skill_overlaps);

        SkillSynergyAnalysis {
            synergy_score: synergy_score.min(1.0),
            complementary_skills,
            skill_gaps,
            skill_overlaps,
            optimization_suggestions,
        }
    }

    fn find_complementary_skills(&self, skills: &[String]) -> Vec<SkillPair> {
        let synergies = vec![
            (("React", "Node.js"), "Full-stack JavaScript development"),
            (("Python", "Machine Learning"), "AI/ML development"),
            (("UI/UX Design", "Frontend"), "Complete user experience"),
            (("Backend", "DevOps"), "End-to-end deployment"),
            (("Mobile", "API"), "Mobile application development"),
        ];

        let mut pairs = Vec::new();
        for ((skill_a, skill_b), reason) in synergies {
            if skills.iter().any(|s| s.contains(skill_a)) && skills.iter().any(|s| s.contains(skill_b)) {
                pairs.push(SkillPair {
                    skill_a: skill_a.to_string(),
                    skill_b: skill_b.to_string(),
                    synergy_level: 0.9,
                    reason: reason.to_string(),
                });
            }
        }
        pairs
    }

    fn generate_optimization_suggestions(
        &self,
        gaps: &[SkillGap],
        overlaps: &[SkillOverlap],
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        if !gaps.is_empty() {
            suggestions.push(format!("Add team member with skills: {}", 
                gaps.iter().map(|g| g.missing_skill.as_str()).collect::<Vec<_>>().join(", ")));
        }

        for overlap in overlaps {
            if overlap.redundancy_level > 0.4 {
                suggestions.push(format!(
                    "Reduce redundancy in {} - consider diversifying skills",
                    overlap.skill
                ));
            }
        }

        if suggestions.is_empty() {
            suggestions.push("Team composition is well-optimized".to_string());
        }

        suggestions
    }

    async fn build_collaboration_matrix(
        &self,
        freelancers: &[FreelancerEmbedding],
    ) -> Result<HashMap<(Uuid, Uuid), f64>, sqlx::Error> {
        let mut matrix = HashMap::new();
        
        // Query past collaborations from database
        for i in 0..freelancers.len() {
            for j in (i + 1)..freelancers.len() {
                let score = self.get_collaboration_score(
                    freelancers[i].freelancer_id,
                    freelancers[j].freelancer_id,
                ).await.unwrap_or(0.5);
                
                matrix.insert((freelancers[i].freelancer_id, freelancers[j].freelancer_id), score);
                matrix.insert((freelancers[j].freelancer_id, freelancers[i].freelancer_id), score);
            }
        }
        
        Ok(matrix)
    }

    async fn get_collaboration_score(&self, freelancer_a: Uuid, freelancer_b: Uuid) -> Result<f64, sqlx::Error> {
        // Query database for past team projects together
        // For now, return default score
        Ok(0.7)
    }

    fn select_next_team_member(
        &self,
        current_team: &[FreelancerEmbedding],
        available: &[FreelancerEmbedding],
        required_skills: &[String],
        covered_skills: &[String],
        collaboration_matrix: &HashMap<(Uuid, Uuid), f64>,
        timezone_pref: Option<&str>,
    ) -> Result<Option<FreelancerEmbedding>, sqlx::Error> {
        let current_ids: Vec<Uuid> = current_team.iter().map(|f| f.freelancer_id).collect();
        
        let mut best_candidate = None;
        let mut best_score = 0.0;

        for candidate in available {
            if current_ids.contains(&candidate.freelancer_id) {
                continue;
            }

            // Calculate value added by this candidate
            let new_skills: Vec<String> = candidate.skills
                .iter()
                .filter(|s| !covered_skills.contains(s) && required_skills.contains(s))
                .cloned()
                .collect();

            let skill_value = new_skills.len() as f64 / required_skills.len() as f64;

            // Calculate collaboration compatibility
            let mut collab_score = 0.0;
            for member in current_team {
                collab_score += collaboration_matrix
                    .get(&(member.freelancer_id, candidate.freelancer_id))
                    .unwrap_or(&0.5);
            }
            collab_score /= current_team.len().max(1) as f64;

            // Timezone compatibility
            let timezone_score = if let Some(pref) = timezone_pref {
                if candidate.timezone.contains(pref) { 1.0 } else { 0.5 }
            } else {
                1.0
            };

            let total_score = skill_value * 0.5 + collab_score * 0.3 + timezone_score * 0.2;

            if total_score > best_score {
                best_score = total_score;
                best_candidate = Some(candidate.clone());
            }
        }

        Ok(best_candidate)
    }

    fn calculate_team_score(
        &self,
        team: &[FreelancerEmbedding],
        required_skills: &[String],
        collaboration_matrix: &HashMap<(Uuid, Uuid), f64>,
        timezone_pref: Option<&str>,
    ) -> f64 {
        let synergy = self.analyze_skill_synergy(team, required_skills);
        let collab = self.calculate_collaboration_score(team, collaboration_matrix);
        let timezone = self.calculate_timezone_compatibility(team, timezone_pref);

        synergy.synergy_score * 0.4 + collab * 0.4 + timezone * 0.2
    }

    fn calculate_collaboration_score(
        &self,
        team: &[FreelancerEmbedding],
        matrix: &HashMap<(Uuid, Uuid), f64>,
    ) -> f64 {
        if team.len() < 2 {
            return 1.0;
        }

        let mut total = 0.0;
        let mut count = 0;

        for i in 0..team.len() {
            for j in (i + 1)..team.len() {
                total += matrix.get(&(team[i].freelancer_id, team[j].freelancer_id)).unwrap_or(&0.5);
                count += 1;
            }
        }

        if count > 0 { total / count as f64 } else { 0.5 }
    }

    fn calculate_timezone_compatibility(&self, team: &[FreelancerEmbedding], pref: Option<&str>) -> f64 {
        if let Some(preferred) = pref {
            let matching = team.iter().filter(|f| f.timezone.contains(preferred)).count();
            matching as f64 / team.len() as f64
        } else {
            // Calculate timezone spread
            let unique_timezones: std::collections::HashSet<_> = team.iter().map(|f| &f.timezone).collect();
            if unique_timezones.len() <= 3 { 0.9 } else { 0.6 }
        }
    }

    fn determine_role(&self, skills: &[String], required: &[String]) -> String {
        for skill in skills {
            if required.contains(skill) {
                return skill.clone();
            }
        }
        "Generalist".to_string()
    }

    fn analyze_team_dynamics(&self, team: &[FreelancerEmbedding], required: &[String]) -> TeamDynamics {
        let avg_experience: f32 = team.iter().map(|f| f.experience_years).sum::<f32>() / team.len() as f32;
        let experience_variance = team.iter()
            .map(|f| (f.experience_years - avg_experience).powi(2))
            .sum::<f32>() / team.len() as f32;

        TeamDynamics {
            leadership_score: team.iter().map(|f| f.experience_years).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0) as f64 / 10.0,
            diversity_score: (team.len() as f64).min(5.0) / 5.0,
            experience_balance: 1.0 - (experience_variance.sqrt() as f64 / 10.0).min(1.0),
            potential_conflicts: vec![],
            strengths: vec![
                "Diverse skill set".to_string(),
                "Complementary expertise".to_string(),
            ],
        }
    }
}
