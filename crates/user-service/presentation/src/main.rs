mod config;
mod dto;
mod handlers;

use application::user_service::UserApplicationService;
use axum::{Router, routing::post};
use config::Env;
use handlers::AppState;
use handlers::create_user_handler;
use infrastructure::PgUserRepository;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Env::from_env()
        .map_err(|e| {
            panic!("Failed to load configuration: {}", e);
        })
        .unwrap();
    info!("Load configuration successfully.");

    let db_url = cfg.database_url;
    let max_conn = cfg.max_connection;
    let min_conn = cfg.min_connection;
    let pool = config::init_connection(&db_url, max_conn, min_conn).await?;
    info!("Connected to the database");
    let conn = Arc::new(pool);

    let user_repo = Arc::new(PgUserRepository::new(Arc::clone(&conn)));
    let user_service = Arc::new(UserApplicationService::new(user_repo));

    let share_state = Arc::new(AppState { user_service });
    let app = Router::new()
        .route("/users", post(create_user_handler))
        .with_state(share_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!(
        "User service is running on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
