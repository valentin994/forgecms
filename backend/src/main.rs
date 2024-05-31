use axum::{response::IntoResponse, routing::get, Router};
use forgecms::error::internal_error;
use reqwest::StatusCode;
use routes::reviews::review_router;
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::net::TcpListener;

mod error;
pub mod routes;

const DB_MAX_CONNECTIONS: u32 = 10;
const DB_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // initialize database
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:mysecretpassword@db:5432".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(DB_MAX_CONNECTIONS)
        .acquire_timeout(DB_CONNECTION_TIMEOUT)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    MIGRATOR.run(&pool).await.unwrap();
    tracing::debug!("Database initialized and migrated");
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest("/review", review_router(pool).await)
        .fallback(handler_404);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .map_err(internal_error)
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    tracing::debug!("hello");
    "Hello, World!"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
