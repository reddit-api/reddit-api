use std::{fmt::Display, io::Read};

use crate::{
    responses::sub_reddit_new::SubRedditNew,
    routes::{EndPoints, Method},
};
use reqwest::{header, Client};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct RedditClient {
    pub(crate) client: Client,
}
impl RedditClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .brotli(true)
            .build()
            .unwrap();
        RedditClient { client }
    }
    pub fn new_with_session<T:fmt::Display, W:fmt::Display>(session: T, user_agent: W) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(session.as_str()).unwrap(),
        );
        headers.insert(
            "user-agent",
            header::HeaderValue::from_str(user_agent.as_str()).unwrap(),
        );

        let client = reqwest::Client::builder()
            .gzip(true)
            .brotli(true)
            .default_headers(headers)
            .build()
            .unwrap();
        RedditClient { client }
    }

    pub async fn req<T: DeserializeOwned>(&self, route: EndPoints) -> Result<T, &str> {
        let res = match route.method() {
            Method::Get => {
                let res = self
                    .client
                    .get(format!("https://reddit.com/{}", route.path()))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                res
            }
            _ => todo!(),
        };

        Ok(serde_json::from_str(&res).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::http::RedditApi;

    #[tokio::test]
    async fn test_new_posts() {
        let reddit = RedditApi::new();
        let res = reddit
            .reddit("bottalks")
            .get_newest_posts("new".to_owned())
            .await;
        //It hasnt crashed yet so it works!
        println!("{}", res.kind);
    }
}
