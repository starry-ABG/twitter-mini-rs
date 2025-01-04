use std::sync::Arc;

use axum::{
    extract::{Extension, Path, State},
    Json
};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::services::tweet_service::TweetService;
use crate::errors::AppError;
use crate::middleware::jwt_extractor::AuthUser;

#[derive(Deserialize)]
pub struct PostTweetRequest {
    pub user_id: i32,
    pub content: String,
}

#[derive(Serialize)]
pub struct PostTweetResponse {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
}

#[derive(Serialize)]
pub struct TweetItem {
    pub id: i32,
    pub content: String,
}

pub async fn post_tweet(
    State(state): State<AppState>,
    auth_user: AuthUser,  // 这里验证JWT (要把路由放在需要AuthUser的地方)
    Json(payload): Json<PostTweetRequest>,
) -> Result<Json<PostTweetResponse>, AppError> {
    // 如果要做更严格的权限验证，可对比auth_user.subject == payload.user_id (string->int)
    let new_tweet = TweetService::post_tweet(
        &state.db,
        payload.user_id,
        &payload.content,
    ).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let resp = PostTweetResponse {
        id: new_tweet.id,
        user_id: new_tweet.user_id,
        content: new_tweet.content,
    };
    Ok(Json(resp))
}

pub async fn list_user_tweets(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<TweetItem>>, AppError> {
    let tweets = TweetService::list_user_tweets(&state.db, user_id)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let data: Vec<TweetItem> = tweets.into_iter()
        .map(|t| TweetItem { id: t.id, content: t.content })
        .collect();

    Ok(Json(data))
}