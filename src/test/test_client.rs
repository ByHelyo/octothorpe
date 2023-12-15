use http::Response;
use url::Url;

use super::MockClient;
use crate::{core::Client, rest_error::RestError};

pub struct TestClient {
    request_line: MockClient,
    response_body: Vec<u8>,
}

impl TestClient {
    pub fn new_raw<T>(expected: MockClient, data: T) -> Self
    where
        T: Into<Vec<u8>>,
    {
        Self {
            request_line: expected,
            response_body: data.into(),
        }
    }

    pub fn response(&self) -> Response<Vec<u8>> {
        Response::builder()
            .status(self.request_line.status)
            .body(self.response_body.clone())
            .unwrap()
    }
}

impl Client for TestClient {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<url::Url, crate::core::ApiError<Self::Error>> {
        Ok(Url::parse(&format!("https://slack.com/api/{}", endpoint))?)
    }

    fn rest(
        &self,
        request: http::request::Builder,
    ) -> Result<http::Response<bytes::Bytes>, crate::core::ApiError<Self::Error>>
    {
        let url = Url::parse(&request.uri_ref().unwrap().to_string()).unwrap();
        self.request_line
            .check(request.method_ref().unwrap().clone(), &url);

        Ok(self.response().map(Into::into))
    }
}
