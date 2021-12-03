use strum::Display;
// https://www.reddit.com/r/bottalks/new.json?sort=new
pub enum EndPoints {
    AccessToken,
    SubRedditNew { sort: String, reddit: String },
    SubmitPost,
}

#[derive(Debug, Display)]

pub enum Method {
    Post,
    Get,
    Head,
    Patch,
    Delete,
}

#[derive(PartialEq)]
pub enum AuthType {
    Oath,
    None,
}

impl EndPoints {
    pub fn path(&self) -> String {
        match self {
            EndPoints::AccessToken => "api/v1/access_token".to_owned(),
            EndPoints::SubRedditNew { sort, reddit } => {
                format!("r/{}/new.json?sort={}", reddit, sort)
            }
            EndPoints::SubmitPost => "api/submit.json".to_owned(),
        }
    }

    pub const fn method(&self) -> Method {
        match *self {
            EndPoints::AccessToken | EndPoints::SubmitPost => Method::Post,
            EndPoints::SubRedditNew { sort: _, reddit: _ } => Method::Get,
        }
    }
    pub const fn auth_type(&self) -> AuthType {
        match *self {
            EndPoints::AccessToken | EndPoints::SubmitPost => AuthType::Oath,
            EndPoints::SubRedditNew { sort: _, reddit: _ } => AuthType::None,
        }
    }
}
