use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait,
    ActiveModelTrait, Set, IntoActiveModel
};
use crate::entities::{users, users::Entity as User};
use crate::entities::users::ActiveModel as UserActiveModel;
use anyhow::Result;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<users::Model>> {
        let user = User::find()
            .filter(users::Column::Username.eq(username))
            .one(db)
            .await?;
        Ok(user)
    }

    pub async fn create_user(
        db: &DatabaseConnection,
        username: &str,
        password_hash: &str,
        nickname: &str,
    ) -> Result<users::Model> {
        let new_user = UserActiveModel {
            username: Set(username.to_string()),
            password_hash: Set(password_hash.to_string()),
            nickname: Set(nickname.to_string()),
            ..Default::default()
        };
        let inserted = new_user.insert(db).await?;
        Ok(inserted)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Option<users::Model>> {
        let user = User::find_by_id(user_id).one(db).await?;
        Ok(user)
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        mut user_model: users::Model,
        nickname: Option<String>,
        avatar_url: Option<String>,
        password_hash: Option<String>,
    ) -> Result<users::Model> {
        if let Some(nick) = nickname {
            user_model.nickname = nick;
        }
        if let Some(url) = avatar_url {
            user_model.avatar_url = Some(url);
        }
        if let Some(pw) = password_hash {
            user_model.password_hash = pw;
        }
        let mut active_model = user_model.into_active_model();
        let updated = active_model.update(db).await?;
        Ok(updated)
    }
}