use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, patch},
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
        .route("/:id", patch(update_review))
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
            RETURNING id, name, review;"#,
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
        Err(_) => Err(AppError::BodyParsingError(
            "Unable to process the body".to_string(),
        )),
    }
}

async fn read_review(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Review>), AppError> {
    tracing::debug!("Fetching review with id {}", &id);
    match sqlx::query_as!(
        Review,
        r#"SELECT id, name, review FROM reviews WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(res) => {
            tracing::debug!("Fetched review! - {res:?}");
            Ok((StatusCode::OK, Json(res)))
        }
        Err(e) => {
            tracing::error!("Encountered error: {}", e);
            Err(AppError::MissingResource(format!(
                "Can't find review with id {}",
                id
            )))
        }
    }
}

async fn update_review(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    payload: Result<Json<UpdateReviw>, JsonRejection>,
) -> Result<(StatusCode, Json<Review>), AppError> {
    match payload {
        Ok(payload) => {
            let res = sqlx::query_as!(
                Review,
                r#"SELECT id, name, review FROM reviews WHERE id = $1"#,
                id
                ).fetch_one(&pool)
                .await
                .expect("hell no");
            tracing::debug!("Fetched the original review: {res:?}");
            
            let update = sqlx::query_as!(
                Review,
                r#"UPDATE reviews SET name = $1, review = $2 WHERE id = $3 RETURNING id, name, review;"#,
                payload.name.clone().unwrap_or(res.name),
                payload.review.clone().unwrap_or(res.review),
                id
                ).fetch_one(&pool)
                .await
                .expect("error");

            Ok((StatusCode::OK, Json(update)))
        },
        Err(e) => {
            tracing::error!("Ecnountered error: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

async fn get_all_reviews(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Review>>), AppError> {
    tracing::debug!("Getting all reviews");
    match sqlx::query_as!(Review, r#"SELECT * FROM reviews"#)
        .fetch_all(&pool)
        .await
    {
        Ok(res) => {
            tracing::debug!("Fetched all reviews!");
            Ok((StatusCode::OK, Json(res)))
        }
        Err(e) => {
            tracing::error!("Encountered error: {}", e);
            Err(AppError::InternalServerError)
        }
    }
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

#[derive(Deserialize, Debug)]
struct UpdateReviw {
    name: Option<String>,
    review: Option<String>
}

#[derive(Serialize, Debug)]
struct Review {
    id: i64,
    name: String,
    review: String,
}
