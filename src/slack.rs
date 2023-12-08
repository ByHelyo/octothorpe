use reqwest::blocking;
use url::Url;

use crate::{auth::Auth, core::Client, rest_error::RestError};

struct Slack {
    client: blocking::Client,
    base_url: Url,
    auth: Auth,
}

impl Slack {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let base_url = Url::parse("https://slack.com/api/").unwrap();

        let client = blocking::Client::new();
        let auth = Auth::Token(token.into());

        Self {
            client,
            base_url,
            auth,
        }
    }
}

impl Client for Slack {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Url, crate::core::ApiError<Self::Error>> {
        Ok(self.base_url.join(endpoint)?)
    }

    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, crate::core::ApiError<Self::Error>>
    {
        let call = || -> Result<_, RestError> { todo!() };
        Ok(call()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_slack_client() {
        let client = Slack::new("mytoken");
        assert_eq!(
            client.base_url,
            Url::parse("https://slack.com/api/").unwrap()
        );
    }
}
