use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    /// How long a login token stays valid, in seconds.
    pub jwt_ttl_seconds: i64,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("APP_PORT").unwrap_or_else(|_| "8800".to_string());
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_ttl_seconds = env::var("JWT_TTL_SECONDS").unwrap_or_else(|_| "3600".to_string());
        AppConfig {
            host,
            port: port.parse().expect("APP_PORT must be a valid number"),
            jwt_secret,
            jwt_ttl_seconds: jwt_ttl_seconds
                .parse()
                .expect("JWT_TTL_SECONDS must be a number"),
        }
    }
}
