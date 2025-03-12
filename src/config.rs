// src/config.rs
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub google_client_id: String,
    // ...
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        
        Ok(Self {
            database_url: cfg.get("DATABASE_URL")?,
            redis_url: cfg.get("REDIS_URL")?,
            jwt_secret: cfg.get("JWT_SECRET")?,
            google_client_id: cfg.get("GOOGLE_CLIENT_ID")?,
        })
    }
}