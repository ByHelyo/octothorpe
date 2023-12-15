use bytes::Bytes;
use http::{request::Builder, Response};
use reqwest::blocking;
use url::Url;

use crate::{
    auth::Auth,
    core::{ApiError, Client},
    rest_error::RestError,
};

pub struct Slack {
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
    ) -> Result<Url, ApiError<Self::Error>> {
        Ok(self.base_url.join(endpoint)?)
    }

    fn rest(
        &self,
        mut request: Builder,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || -> Result<_, RestError> {
            self.auth.set_header(request.headers_mut().unwrap())?;
            let http_req = request.body(Vec::new())?;
            let request = http_req.try_into()?;
            let rsp = self.client.execute(request)?;

            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, val) in rsp.headers() {
                headers.insert(key, val.clone());
            }
            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        Ok(call()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_rest_endpoint() {
        let client = Slack::new("mytoken");
        let url = client.rest_endpoint("api.test");
        assert_eq!(
            url.unwrap(),
            Url::parse("https://slack.com/api/api.test").unwrap()
        );
    }

    #[test]
    fn invalid_rest_endpoint() {
        let client = Slack::new("mytoken");
        let url = client.rest_endpoint("//").unwrap_err();

        dbg!(&url);
        print!("{}", url);
        assert!(matches!(url, ApiError::UrlParse { .. }));
    }
}
