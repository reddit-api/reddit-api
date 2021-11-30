use std::{fmt::Display};

use reqwest::{Client, header};
use serde::de::DeserializeOwned;

use crate::{
    routes::{EndPoints, Method},
};

#[derive(Clone)]
pub struct RedditClient {
    pub(crate) client: Client,
}

impl RedditClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_session<T: Display, W: Display>(session: T, user_agent: W) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(format!("{}", session).as_str()).unwrap(),
        );
        headers.insert(
            "user-agent",
            header::HeaderValue::from_str(format!("{}", user_agent).as_str()).unwrap(),
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

impl Default for RedditClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::builder()
            .gzip(true)
            .brotli(true)
            .build()
            .unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::http::RedditApi;

    #[tokio::test]
    async fn test_new_posts() {
        let reddit = RedditApi::default();
        let res = reddit
            .reddit("bottalks")
            .get_newest_posts("new".to_owned())
            .await;
        //It hasn't crashed yet so it works!
        println!("{}", res.kind);
    }
}
