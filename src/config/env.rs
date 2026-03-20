use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevMode {
    Development,
    Test,
    Production,
}

impl DevMode {
    fn from_env_value(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "development" | "dev" => Self::Development,
            "test" => Self::Test,
            "production" | "prod" => Self::Production,
            _ => panic!("DEV_MODE must be one of: development|dev, test, production|prod"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvConfig {
    pub dev_mode: DevMode,
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        let dev_mode_raw = env::var("DEV_MODE").unwrap_or_else(|_| "development".to_string());
        let dev_mode = DevMode::from_env_value(&dev_mode_raw);
        let main_database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let test_database_url =
            env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "5000".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid number");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        // Choose the appropriate database URL based on the development mode
        let database_url = if dev_mode == DevMode::Test {
            test_database_url
        } else {
            main_database_url
        };

        EnvConfig {
            dev_mode,
            database_url,
            port,
            jwt_secret,
        }
    }
}
