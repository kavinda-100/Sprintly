use sqlx::PgPool;

pub mod env;
use crate::config::env::EnvConfig;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub app_name: String,
    pub db: PgPool,
    pub env_config: EnvConfig,
}
