pub mod tweet;
pub mod user;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::BoxError;
use axum::Router;
use std::time::Duration;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;

use crate::app_state::AppState;
use crate::routes::tweet::*;
use crate::routes::user::*;

pub fn create_routes() -> Router<AppState> {
    let router = Router::new()
        .route("/user/register", post(register))
        .route("/user/login", post(login))
        .route("/user/update_profile", post(update_profile))
        .route("/tweet/post", post(post_tweet))
        .route("/tweet/list/:user_id", get(list_user_tweets))
        // 挂载限流中间件
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
        );
    router
}
