use std::fmt::{self, Display};
use strum::Display;
// /https://www.reddit.com/r/bottalks/new.json?sort=new
pub enum EndPoints {
    AccessToken,
    SubRedditNew { sort: String, reddit: String },
}

#[derive(Debug, Display)]

pub enum Method {
    Post,
    Get,
    Head,
    Patch,
    Delete,
}

pub enum AuthType {
    Oath,
    None,
}

impl EndPoints {
    pub fn path(&self) -> String {
        match self {
            EndPoints::AccessToken => "api/v1/access_token".to_string(),
            EndPoints::SubRedditNew { sort, reddit } => {
                format!("r/{}/new.json?sort={}", reddit, sort)
            }
        }
    }

    pub const fn method(&self) -> Method {
        match self {
            &EndPoints::AccessToken => Method::Post,
            &EndPoints::SubRedditNew { sort: _, reddit: _ } => Method::Get,
        }
    }
    pub const fn auth_type(&self) -> AuthType {
        match self {
            &EndPoints::AccessToken => AuthType::Oath,
            &EndPoints::SubRedditNew { sort: _, reddit: _ } => AuthType::None,
        }
    }
}
