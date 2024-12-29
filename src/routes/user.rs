use std::sync::Arc;

use axum::extract::State;
use axum::{
    extract::Extension,
    Json
};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::domain::user_service::UserService;
use crate::auth::jwt::generate_token;
use crate::error::AppError;
use crate::entities::users::Model as UserModel;
use anyhow::Result;

// DTO
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub username: String,
    pub nickname: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub token: String, // 返回JWT
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub user_id: i32,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub new_password: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateProfileResponse {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub avatar_url: Option<String>,
}

// Handler

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    let user = UserService::register(
        &state.db,
        &payload.username,
        &payload.password,
        &payload.nickname,
    ).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let resp = RegisterResponse {
        id: user.id,
        username: user.username,
        nickname: user.nickname,
    };
    Ok(Json(resp))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = UserService::login(&state.db, &payload.username, &payload.password)
        .await
        .map_err(|e| AppError::Unauthorized(e.to_string()))?;

    // 生成JWT
    let token = generate_token(
        &state.config.jwt_secret_key,
        state.config.jwt_expire_hours,
        &user.id.to_string(), // sub
    ).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let resp = LoginResponse {
        id: user.id,
        username: user.username,
        nickname: user.nickname,
        token,
    };
    Ok(Json(resp))
}

pub async fn update_profile(
    State(state): State<AppState>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>, AppError> {
    let user = UserService::update_profile(
        &state.db,
        payload.user_id,
        payload.nickname,
        payload.avatar_url,
        payload.new_password,
    ).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let resp = UpdateProfileResponse {
        id: user.id,
        username: user.username,
        nickname: user.nickname,
        avatar_url: user.avatar_url,
    };
    Ok(Json(resp))
}