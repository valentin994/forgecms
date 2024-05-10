use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use axum::{routing::{get, post}, Router, Json, http::StatusCode, extract::Path};

pub async fn review_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", post(create_review))
        .route("/:id", get(read_review))
        .with_state(pool)
}

async fn create_review(Json(_payload):Json<CreateReview>) -> (StatusCode, Json<Review>){
    let review = Review {
        id: 1,
        name: "Hello".to_string(),
        review: "World".to_string()
    };

    tracing::debug!("Created a review!");
    (StatusCode::CREATED, Json(review))
}

async fn read_review(Path(id): Path<u64>) -> (StatusCode, Json<Review>){
    let review = Review {
        id: 1,
        name: "Hello".to_string(),
        review: "World".to_string()
    };
    tracing::debug!("Fetched review!");
    (StatusCode::OK, Json(review))
}

#[derive(Deserialize)]
struct CreateReview {
    name: String,
    review: String
}

#[derive(Serialize)]
struct Review {
    id: u64,
    name: String,
    review: String
}
