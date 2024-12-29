use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret_key: String,
    pub jwt_expire_hours: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL not set in .env"),
            redis_url: env::var("REDIS_URL")
                .expect("REDIS_URL not set in .env"),
            jwt_secret_key: env::var("JWT_SECRET_KEY")
                .unwrap_or_else(|_| "mysecret".into()),
            jwt_expire_hours: env::var("JWT_EXPIRE_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24),
        }
    }
}