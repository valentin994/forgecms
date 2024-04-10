use axum::{routing::get, Router};
use tokio::net::TcpListener;
use sqlx::{migrate::Migrator, postgres::{PgPool, PgPoolOptions}};
use std::time::Duration;

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
    let db_connection_str = std::env::var("localhost:5432")
        .unwrap_or_else(|_| "postgres://postgres:mysecretpassword@localhost:5432".to_string());
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
        // `GET /` goes to `root`
        .route("/", get(root))
        .with_state(pool);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    tracing::debug!("hello");
    "Hello, World!"
}
