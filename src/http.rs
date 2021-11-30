use reqwest::{header::USER_AGENT, Client};

use crate::{
    client::RedditClient, responses::AccessToken,
    subreddit::SubReddit,
};
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

pub struct RedditApi {
    client: RedditClient,
    session: Option<Session>,
}
impl RedditApi {
    pub async fn login(
        &mut self,
        user_agent: String,
        client_id: String,
        client_secret: String,
        username: String,
        password: String,
    ) {
        let form = [
            ("grant_type", "password"),
            ("username", &username),
            ("password", &password),
        ];
        dbg!(&user_agent);
        dbg!(&client_id);
        dbg!(&client_secret);
        dbg!(&username);
        dbg!(&password);

        let req = Client::new()
            .post("https://www.reddit.com/api/v1/access_token".to_owned())
            .header(USER_AGENT, &user_agent)
            .basic_auth(&client_id, Some(&client_secret))
            .form(&form);

        let res = req.send().await.unwrap();

        if res.status() == 200 {
            let json = res.json::<AccessToken>().await.unwrap();
            self.session = Some(Session {
                user_agent: user_agent.clone(),
                client_id,
                client_secret,
                username,
                password,
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

impl Default for RedditApi {
    fn default() -> Self {
        Self {
            client: RedditClient::new(),
            ..Default::default()
        }
    }
}
