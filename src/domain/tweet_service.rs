use crate::repository::tweet_repo::TweetRepository;
use crate::entities::tweets;
use sea_orm::DatabaseConnection;
use anyhow::Result;

pub struct TweetService;

impl TweetService {
    pub async fn post_tweet(
        db: &DatabaseConnection,
        user_id: i32,
        content: &str,
    ) -> Result<tweets::Model> {
        let new_tweet = TweetRepository::create_tweet(db, user_id, content).await?;
        Ok(new_tweet)
    }

    pub async fn list_user_tweets(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<tweets::Model>> {
        TweetRepository::find_by_user_id(db, user_id).await
    }
}