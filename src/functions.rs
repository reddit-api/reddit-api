use crate::{client::RedditClient, responses::SubRedditNew, routes::EndPoints};

// Helper function to be used in the struct for easier use
pub async fn get_newest_posts(client: &RedditClient, reddit: String, sort: String) -> SubRedditNew {
    let route = EndPoints::SubRedditNew { reddit, sort };
    client.req::<SubRedditNew>(route).await.unwrap()
}
