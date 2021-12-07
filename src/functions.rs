use serde_json::Value;

use crate::{client::RedditClient, responses::SubRedditNew, routes::EndPoints};

// Helper function to be used in the struct for easier use
pub async fn get_newest_posts(client: &RedditClient, reddit: String, sort: String) -> SubRedditNew {
    let route = EndPoints::SubRedditNew { reddit, sort };
    client.req::<SubRedditNew, ()>(route, None).await.unwrap()
}

pub async fn submit_post_text(
    client: &RedditClient,
    sub_reddit: String,
    title: String,
    text: String,
) {
    let form = [
        ("kind", "self"),
        ("title", &title),
        ("text", &text),
        ("sr", &sub_reddit),
    ];
    println!("{:#?}", form);
    let r = client
        .req::<Value, _>(EndPoints::SubmitPost, Some(form))
        .await;
    let r = serde_json::to_string(&r.unwrap()).unwrap();
    println!("{}", r);
    std::fs::write("out.json", r).unwrap();
}

pub async fn submit_post_url(
    client: &RedditClient,
    sub_reddit: String,
    title: String,
    url: String,
) {
    let form = [
        ("kind", "self"),
        ("title", &title),
        ("link", &url),
        ("sr", &sub_reddit),
    ];
    let r = client
        .req::<Value, _>(EndPoints::SubmitPost, Some(form))
        .await;
    let r = serde_json::to_string(&r.unwrap()).unwrap();
    println!("{}", r);
    std::fs::write("out.json", r).unwrap();
}
