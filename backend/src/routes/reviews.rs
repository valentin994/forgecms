use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use crate::error::AppError;

pub async fn review_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", post(create_review))
        .route("/", get(get_all_reviews))
        .route("/:id", get(read_review))
        .fallback(handler_404)
        .with_state(pool)
}

async fn create_review(
    State(pool): State<PgPool>,
    payload: Result<Json<CreateReview>, JsonRejection>,
) -> Result<(StatusCode, Json<Review>), AppError> {
    match payload {
        Ok(payload) => {
            tracing::debug!(
                "Payload: name: {}, review: {}",
                &payload.name,
                &payload.review
            );
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
            .await
            .expect("Can't insert into database");
            tracing::debug!("{res:?}");
            tracing::debug!("Created a review!");
            Ok((StatusCode::CREATED, Json(res)))
        }
        Err(_) => Err(AppError::BodyParsingError("Unable to process the body".to_string())),
    }
}

async fn read_review(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<Review>) {
    tracing::debug!("Fetching review with id {}", &id);
    let res = sqlx::query_as!(
        Review,
        r#"SELECT id, name, review FROM reviews WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .expect("Can't fetch review");
    tracing::debug!("Fetched review! - {res:?}");
    (StatusCode::OK, Json(res))
}

async fn get_all_reviews(State(pool): State<PgPool>) -> (StatusCode, Json<Vec<Review>>) {
    tracing::debug!("Getting all reviews");
    let res = sqlx::query_as!(Review, r#"SELECT * FROM reviews"#)
        .fetch_all(&pool)
        .await
        .expect("");
    tracing::debug!("Fetched all reviews!");
    (StatusCode::OK, Json(res))
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
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
