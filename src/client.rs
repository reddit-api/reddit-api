use std::fmt::Display;

use reqwest::{header, Client};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    routes::{AuthType, EndPoints, Method},
};

#[derive(Clone, Default)]
pub struct RedditClient {
    pub(crate) client: Client,
    pub(crate) session: bool,
}

impl RedditClient {
    pub fn new() -> Self {
        RedditClient {
            client: reqwest::Client::builder()
                .gzip(true)
                .brotli(true)
                .build()
                .unwrap(),
            session: false,
        }
    }

    pub fn new_with_session<T: Display, W: Display>(session: T, user_agent: W) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", session)).unwrap(),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&user_agent.to_string()).unwrap(),
        );
        let client = reqwest::Client::builder()
            .gzip(true)
            .brotli(true)
            .default_headers(headers)
            .build()
            .unwrap();
        RedditClient {
            client,
            session: true,
        }
    }

    pub async fn req<T: DeserializeOwned, F>(
        &self,
        route: EndPoints,
        form: Option<F>,
    ) -> Result<T, &str>
    where
        F: Serialize + Sized,
    {
        let url = format!(
            "https://{}reddit.com/{}",
            if route.auth_type() == AuthType::Oauth {
                "oauth."
            } else {
                "www."
            },
            route.path()
        );
        println!("URL: {} METHOD: {}", &url, route.method());
        let req = match route.method() {
            Method::Get => self.client.get(url),
            Method::Delete => self.client.delete(url),
            Method::Head => self.client.head(url),
            Method::Patch => self.client.patch(url),
            Method::Post => self.client.post(url),
        };

        let req = if let Some(form) = form {
            req.form(&form)
        } else {
            req
        };
        let res = req.send().await.unwrap().text().await.unwrap();
        println!("{}", res);
        std::fs::write("out.json", &res).unwrap();

        Ok(serde_json::from_str(&res).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{http::RedditApi, responses::SubRedditNew};

    #[tokio::test]
    async fn new_posts() {
        let reddit = RedditApi::new();
        let _res: SubRedditNew = reddit
            .reddit("bottalks")
            .get_newest_posts("new".to_owned())
            .await;
    }
}
