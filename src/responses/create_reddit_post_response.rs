use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct CreateRedditPostResponse {
    pub success: bool,
    pub jquery: Value,
}
#[derive(Serialize, Deserialize)]
pub struct RedditPostResponse {
    pub(crate) id: String,
    pub(crate) sub_reddit: String,
    pub(crate) title: String,
}
impl RedditPostResponse {
    pub fn url(&self) -> String {
        format!(
            "https://www.reddit.com/r/{}/comments/{}/{}/",
            self.sub_reddit, self.id, self.title
        )
    }
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn sub_reddit(&self) -> String {
        self.sub_reddit.to_string()
    }
    pub fn title(&self) -> String {
        self.title.to_string()
    }
}
