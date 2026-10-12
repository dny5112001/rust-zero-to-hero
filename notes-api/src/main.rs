use notes_api::app::{AppState, create_app};
use notes_api::config::{AppConfig, DatabaseConfig};
use notes_api::db::create_pool;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "notes_api=info,tower_http=info".into()),
        )
        .init();

    let config = AppConfig::from_env();
    let db_config = DatabaseConfig::from_env();

    let pool = create_pool(&db_config.url)
        .await
        .expect("Failed to create database pool");

    // Bring the database schema up to date. Migrations are compiled into the binary.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let addr = format!("{}:{}", config.host, config.port);
    let state = AppState { db: pool, config };
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {addr} (docs at /docs)");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Wait for Ctrl+C or SIGTERM. Kubernetes sends SIGTERM before stopping a Pod;
/// finishing in-flight requests first gives zero-downtime deploys.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for Ctrl+C")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to listen for SIGTERM")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    // Kubernetes sends SIGTERM and removes the Pod from the Service at the same
    // time, and the ingress takes a moment to notice. Keep serving for a few
    // seconds so requests already on their way don't fail.
    let delay: u64 = std::env::var("SHUTDOWN_DELAY_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    if delay > 0 {
        tracing::info!("shutdown requested; draining for {delay}s");
        tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
    }
    tracing::info!("shutting down gracefully");
}
