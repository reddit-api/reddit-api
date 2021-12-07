use regex::Regex;
use serde_json::Value;

use crate::{
    client::RedditClient,
    responses::{CreateRedditPostResponse, RedditPostResponse, SubRedditNew},
    routes::EndPoints,
};
lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#""https://www\.reddit\.com/r/(.*)/""#).unwrap();
}
// Helper function to be used in the struct for easier use
pub async fn get_newest_posts(client: &RedditClient, reddit: String, sort: String) -> SubRedditNew {
    let route = EndPoints::SubRedditNew { reddit, sort };
    client.req::<SubRedditNew, ()>(route, None).await.unwrap()
}
//TODO share code between these 2 functions
pub async fn submit_post_text(
    client: &RedditClient,
    sub_reddit: String,
    title: String,
    text: String,
) -> RedditPostResponse {
    let form = [
        ("kind", "self"),
        ("title", &title),
        ("text", &text),
        ("sr", &sub_reddit),
    ];

    let r = client
        .req::<CreateRedditPostResponse, _>(EndPoints::SubmitPost, Some(form))
        .await
        .unwrap();
    let str = &serde_json::to_string(&r.jquery).unwrap();
    let captures = RE.captures(str).unwrap();
    let mut matches = captures.get(1).unwrap().as_str().split("/");

    let (id, title) = (matches.nth(2).unwrap(), matches.next().unwrap());
    //TODO change this to a function
    RedditPostResponse {
        id: id.to_string(),
        title: title.to_string(),
        sub_reddit,
    }
}

pub async fn submit_post_url(
    client: &RedditClient,
    sub_reddit: String,
    title: String,
    url: String,
) -> RedditPostResponse {
    let form = [
        ("kind", "self"),
        ("title", &title),
        ("link", &url),
        ("sr", &sub_reddit),
    ];

    let r = client
        .req::<CreateRedditPostResponse, _>(EndPoints::SubmitPost, Some(form))
        .await
        .unwrap();
    let str = &serde_json::to_string(&r.jquery).unwrap();
    let captures = RE.captures(str).unwrap();
    let mut matches = captures.get(1).unwrap().as_str().split("/");

    let (id, title) = (matches.nth(2).unwrap(), matches.next().unwrap());
    //TODO change this to a function
    RedditPostResponse {
        id: id.to_string(),
        title: title.to_string(),
        sub_reddit,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    pub async fn test_regex() {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r#""https://www\.reddit\.com/r/(.*)/""#).unwrap();
        }

        let captures = RE.captures(
            r#""{
	"jquery": [
		[0, 1, "call", ["body"]],
		[1, 2, "attr", "find"],
		[2, 3, "call", [".status"]],
		[3, 4, "attr", "hide"],
		[4, 5, "call", []],
		[5, 6, "attr", "html"],
		[6, 7, "call", [""]],
		[7, 8, "attr", "end"],
		[8, 9, "call", []],
		[1, 10, "attr", "redirect"],
		[
			10,
			11,
			"call",
			["https://www.reddit.com/r/reddit_api_bot/comments/rb0a2o/balls/"]
		],
		[1, 12, "attr", "find"],
		[12, 13, "call", ["*[name=url]"]],
		[13, 14, "attr", "val"],
		[14, 15, "call", [""]],
		[15, 16, "attr", "end"],
		[16, 17, "call", []],
		[1, 18, "attr", "find"],
		[18, 19, "call", ["*[name=text]"]],
		[19, 20, "attr", "val"],
		[20, 21, "call", [""]],
		[21, 22, "attr", "end"],
		[22, 23, "call", []],
		[1, 24, "attr", "find"],
		[24, 25, "call", ["*[name=title]"]],
		[25, 26, "attr", "val"],
		[26, 27, "call", [" "]],
		[27, 28, "attr", "end"],
		[28, 29, "call", []]
	],
	"success": true
}
""#,
        );
        let wrapped = captures.unwrap();
        let mut matches = wrapped.get(1).unwrap().as_str().split("/");

        println!(
            "ID: {} NAME: {}",
            matches.nth(2).unwrap(),
            matches.next().unwrap()
        )
    }
}
