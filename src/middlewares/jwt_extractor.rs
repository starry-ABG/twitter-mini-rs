use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode}
};
use crate::error::AppError;
use crate::auth::jwt::verify_token;
use crate::app_state::AppState;
use std::sync::Arc;
use tower::BoxError;

#[derive(Debug)]
pub struct AuthUser {
    pub subject: String,  // 这里可能是 user_id 或 username
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // 读取请求头
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid Authorization header".into()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid Authorization scheme".into()));
        }
        let token = auth_header.trim_start_matches("Bearer ");

        // 验证 token
        let claims = verify_token(&state.config.jwt_secret_key, token)
            .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

        Ok(AuthUser { subject: claims.sub })
    }
}