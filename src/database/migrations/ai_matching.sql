
-- AI Matching Engine Tables

-- Store match history for learning and analytics
CREATE TABLE IF NOT EXISTS match_history (
    id SERIAL PRIMARY KEY,
    freelancer_id UUID NOT NULL REFERENCES freelancer_accounts(id),
    project_id INTEGER NOT NULL REFERENCES jobs(id),
    match_score FLOAT NOT NULL,
    skill_match FLOAT NOT NULL,
    experience_match FLOAT NOT NULL,
    budget_fit FLOAT NOT NULL,
    success_probability FLOAT NOT NULL,
    was_hired BOOLEAN DEFAULT FALSE,
    project_success BOOLEAN,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Store skill embeddings for faster matching
CREATE TABLE IF NOT EXISTS skill_embeddings (
    skill_name VARCHAR(255) PRIMARY KEY,
    embedding FLOAT[] NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Store freelancer performance metrics
CREATE TABLE IF NOT EXISTS freelancer_metrics (
    freelancer_id UUID PRIMARY KEY REFERENCES freelancer_accounts(id),
    total_projects INTEGER DEFAULT 0,
    completed_projects INTEGER DEFAULT 0,
    completion_rate FLOAT DEFAULT 0.0,
    avg_rating FLOAT DEFAULT 0.0,
    on_time_delivery_rate FLOAT DEFAULT 0.0,
    communication_score FLOAT DEFAULT 0.0,
    response_time_hours FLOAT DEFAULT 24.0,
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Store project complexity scores
CREATE TABLE IF NOT EXISTS project_complexity (
    project_id INTEGER PRIMARY KEY REFERENCES jobs(id),
    complexity_score FLOAT NOT NULL,
    required_experience_years FLOAT DEFAULT 0.0,
    estimated_hours INTEGER DEFAULT 0,
    calculated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_match_history_freelancer ON match_history(freelancer_id);
CREATE INDEX IF NOT EXISTS idx_match_history_project ON match_history(project_id);
CREATE INDEX IF NOT EXISTS idx_match_history_score ON match_history(match_score DESC);
CREATE INDEX IF NOT EXISTS idx_freelancer_metrics_rating ON freelancer_metrics(avg_rating DESC);
CREATE INDEX IF NOT EXISTS idx_freelancer_metrics_completion ON freelancer_metrics(completion_rate DESC);
