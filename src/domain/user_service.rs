use crate::repository::user_repo::UserRepository;
use crate::entities::users;
use sea_orm::DatabaseConnection;
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct UserService;

impl UserService {
    pub async fn register(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
        nickname: &str,
    ) -> Result<users::Model> {
        if UserRepository::find_by_username(db, username).await?.is_some() {
            return Err(anyhow!("Username already exists"));
        }
        let password_hash = hash(password, DEFAULT_COST)?;
        let user = UserRepository::create_user(db, username, &password_hash, nickname).await?;
        Ok(user)
    }

    pub async fn login(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
    ) -> Result<users::Model> {
        let user = UserRepository::find_by_username(db, username)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;
        let valid = verify(password, &user.password_hash)?;
        if !valid {
            return Err(anyhow!("Invalid password"));
        }
        Ok(user)
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        user_id: i32,
        nickname: Option<String>,
        avatar_url: Option<String>,
        new_password: Option<String>,
    ) -> Result<users::Model> {
        let found = UserRepository::find_by_id(db, user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;
        let new_hash = if let Some(pw) = new_password {
            Some(hash(&pw, DEFAULT_COST)?)
        } else {
            None
        };
        let updated = UserRepository::update_user(db, found, nickname, avatar_url, new_hash).await?;
        Ok(updated)
    }
}