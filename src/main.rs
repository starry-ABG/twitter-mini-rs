mod app_state;
mod config;
mod error;
mod middlewares {
    pub mod jwt_extractor;
}
mod auth {
    pub mod jwt;
}
mod domain;
mod entities;
mod log;
mod repository;
mod routes;
mod cache;

use crate::app_state::AppState;
use crate::config::Config;
use crate::routes::create_routes;
use axum::{Extension, Router};
use deadpool_redis::Config as RedisConfig;
use deadpool_redis::Runtime;
use log::init_tracing;
use sea_orm::Database;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // 初始化日志
    let _guard = init_tracing()?;
    // tracing_subscriber::registry()
    //     .with(EnvFilter::from_default_env()) // RUST_LOG=info
    //     .with(fmt::layer())
    //     .init();

    // 加载配置
    let config = Config::from_env();
    tracing::info!("Loaded config: {:?}", config);

    // 连接数据库
    let db = Database::connect(&config.database_url).await?;
    tracing::info!("Connected to PostgreSQL");

    // 初始化 Redis 连接池
    let redis_cfg = RedisConfig::from_url(config.redis_url.clone());
    let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1))?;

    // 全局状态
    let state = AppState {
        db,
        redis_pool,
        config,
    };
    // let state = Arc::new(state);

    // 构建路由
    let app = create_routes().with_state(state)
        // .with_state(state)
        // .layer(Extension(state))
        // 追踪日志
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
