use reqwest::{header::USER_AGENT, Client};

use crate::{client::RedditClient, responses::AccessToken, subreddit::SubReddit};
use core::fmt;
use std::fmt::Display;

#[allow(dead_code)]
pub struct Session {
    user_agent: String,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    session_id: String,
}

#[derive(Default)]
pub struct RedditApi {
    client: RedditClient,
    session: Option<Session>,
}
impl RedditApi {
    pub fn new() -> Self {
        RedditApi {
            client: RedditClient::new(),
            session: None,
        }
    }

    pub async fn login<
        A: fmt::Display,
        B: fmt::Display,
        C: fmt::Display,
        D: fmt::Display,
        E: fmt::Display,
    >(
        &mut self,
        user_agent: A,
        client_id: B,
        client_secret: C,
        username: D,
        password: E,
    ) {
        let form = [
            ("grant_type", "password"),
            ("username", &username.to_string()),
            ("password", &password.to_string()),
        ];

        let req = Client::new()
            .post("https://www.reddit.com/api/v1/access_token".to_owned())
            .header(USER_AGENT, &user_agent.to_string())
            .basic_auth(&client_id, Some(&client_secret))
            .form(&form);
        let res = req.send().await.unwrap();

        if res.status() == 200 {
            let json = res.json::<AccessToken>().await.unwrap();
            self.session = Some(Session {
                user_agent: user_agent.to_string(),
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                username: username.to_string(),
                password: password.to_string(),
                session_id: json.access_token.clone(),
            });
            self.client = RedditClient::new_with_session(&json.access_token, &user_agent);
        } else {
            todo!()
        };
    }
    pub fn reddit<T: Display>(&self, reddit: T) -> SubReddit {
        SubReddit::new(self.client.clone(), reddit.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    pub async fn test_posting() {
        let mut api = RedditApi::new();
        api.login(
            "linux:Reddit-api:v0.1.0 (by /u/tricked-dev)",
            dotenv::var("CLIENT_ID").unwrap(),
            dotenv::var("CLIENT_SECRET").unwrap(),
            dotenv::var("REDDIT_USERNAME").unwrap(),
            dotenv::var("REDDIT_PASSWORD").unwrap(),
        )
        .await;

        let post = api
            .reddit("reddit_api_bot")
            .submit_post_url("balls", "https://tricked.pro")
            .await;

        println!("id: {} title: {}", post.id(), post.title());
    }
}
