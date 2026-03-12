use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnvConfig {
    pub database_url: String,
    pub port: u16,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "5000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");

        EnvConfig { database_url, port }
    }
}
