use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

pub async fn review_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", post(create_review))
        .route("/:id", get(read_review))
        .with_state(pool)
}

async fn create_review(State(pool): State<PgPool> ,Json(payload): Json<CreateReview>) -> (StatusCode, Json<Review>) {
    let review = Review {
        id: 1,
        name: "Hello".to_string(),
        review: "World".to_string(),
    };
    tracing::debug!("Payload: name: {}, review: {}", &payload.name, &payload.review);
    let res = sqlx::query_as!(
        Review,
        r#"INSERT INTO reviews
        (name, review)
        VALUES ($1, $2)
        RETURNING id, name, review"#,
        payload.name,
        payload.review
    )
    .fetch_one(&pool)
    .await;
    tracing::debug!("{res:?}");
    tracing::debug!("Created a review!");
    (StatusCode::CREATED, Json(review))
}

async fn read_review(Path(id): Path<u64>) -> (StatusCode, Json<Review>) {
    let review = Review {
        id: 1,
        name: "Hello".to_string(),
        review: "World".to_string(),
    };
    tracing::debug!("Fetched review!");
    (StatusCode::OK, Json(review))
}

#[derive(Deserialize, Debug)]
struct CreateReview {
    name: String,
    review: String,
}

#[derive(Serialize, Debug)]
struct Review {
    id: i64,
    name: String,
    review: String,
}
