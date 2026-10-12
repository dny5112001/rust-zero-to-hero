use std::env;

pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DatabaseConfig { url }
    }
}
