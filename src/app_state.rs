use sea_orm::DatabaseConnection;
use deadpool_redis::Pool as RedisPool;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis_pool: RedisPool,
    pub config: Config,
}