
-- Proposal analysis history
CREATE TABLE IF NOT EXISTS proposal_analyses (
    id SERIAL PRIMARY KEY,
    freelancer_id UUID NOT NULL,
    job_id INTEGER NOT NULL,
    estimated_win_rate DOUBLE PRECISION NOT NULL,
    structure_score DOUBLE PRECISION NOT NULL,
    relevance_score DOUBLE PRECISION NOT NULL,
    clarity_score DOUBLE PRECISION NOT NULL,
    professionalism_score DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Job categorizations
CREATE TABLE IF NOT EXISTS job_categorizations (
    id SERIAL PRIMARY KEY,
    job_id INTEGER NOT NULL UNIQUE,
    primary_category VARCHAR(255) NOT NULL,
    subcategories TEXT[] DEFAULT '{}',
    required_skills TEXT[] DEFAULT '{}',
    optional_skills TEXT[] DEFAULT '{}',
    project_type VARCHAR(100) NOT NULL,
    complexity_level VARCHAR(50) NOT NULL,
    min_budget DOUBLE PRECISION,
    max_budget DOUBLE PRECISION,
    team_suitable BOOLEAN DEFAULT FALSE,
    estimated_duration VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Search history for analytics
CREATE TABLE IF NOT EXISTS search_history (
    id SERIAL PRIMARY KEY,
    user_id UUID,
    query TEXT NOT NULL,
    search_type VARCHAR(50) NOT NULL,
    results_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Trending skills
CREATE TABLE IF NOT EXISTS trending_skills (
    id SERIAL PRIMARY KEY,
    skill_name VARCHAR(255) NOT NULL UNIQUE,
    demand_score DOUBLE PRECISION NOT NULL,
    growth_rate DOUBLE PRECISION NOT NULL,
    avg_rate DOUBLE PRECISION,
    last_updated TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_proposal_analyses_freelancer ON proposal_analyses(freelancer_id);
CREATE INDEX idx_proposal_analyses_job ON proposal_analyses(job_id);
CREATE INDEX idx_job_categorizations_job ON job_categorizations(job_id);
CREATE INDEX idx_search_history_user ON search_history(user_id);
CREATE INDEX idx_trending_skills_name ON trending_skills(skill_name);
