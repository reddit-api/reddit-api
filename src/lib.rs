pub(crate) mod client;
pub mod functions;
pub(crate) mod http;
pub mod responses;
pub use http::RedditApi;
pub(crate) mod routes;
pub(crate) mod subreddit;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
