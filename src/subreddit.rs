use core::fmt;

use crate::{
    client::RedditClient,
    functions::{get_newest_posts, submit_post_text},
    responses::SubRedditNew,
};

pub struct SubReddit {
    client: RedditClient,
    subreddit: String,
}
impl SubReddit {
    pub fn new(client: RedditClient, reddit: String) -> Self {
        SubReddit {
            client,
            subreddit: reddit,
        }
    }

    pub async fn submit_post_text<T: fmt::Display, V: fmt::Display>(&self, title: T, content: V) {
        submit_post_text(
            &self.client,
            self.subreddit.clone(),
            title.to_string(),
            content.to_string(),
        )
        .await;
    }

    /// Gets the newest posts from a subreddit
    ///  
    /// ```
    /// use crate::RedditApi;
    /// #[tokio::main]
    /// async fn main() {
    ///     let reddit = RedditApi::new();
    ///     let res = reddit
    ///         .reddit("bottalks")
    ///         .get_newest_posts("new".to_owned())
    ///         .await;
    /// }
    /// ```
    pub async fn get_newest_posts(&self, sorting: String) -> SubRedditNew {
        get_newest_posts(&self.client, self.subreddit.clone(), sorting).await
    }
}
