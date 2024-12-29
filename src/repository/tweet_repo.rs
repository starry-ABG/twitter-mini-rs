use sea_orm::{
    DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait
};
use crate::entities::{tweets, tweets::Entity as Tweet};
use crate::entities::tweets::ActiveModel as TweetActiveModel;
use anyhow::Result;

pub struct TweetRepository;

impl TweetRepository {
    pub async fn create_tweet(
        db: &DatabaseConnection,
        user_id: i32,
        content: &str,
    ) -> Result<tweets::Model> {
        let new_tweet = TweetActiveModel {
            user_id: Set(user_id),
            content: Set(content.to_string()),
            ..Default::default()
        };
        let inserted = new_tweet.insert(db).await?;
        Ok(inserted)
    }

    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<tweets::Model>> {
        let tweets = Tweet::find()
            .filter(tweets::Column::UserId.eq(user_id))
            .all(db)
            .await?;
        Ok(tweets)
    }
}