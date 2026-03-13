use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnvConfig {
    pub dev_mode: String,
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        let dev_mode = env::var("DEV_MODE").unwrap_or_else(|_| "development".to_string());
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "5000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        EnvConfig {
            dev_mode,
            database_url,
            port,
            jwt_secret,
        }
    }
}
