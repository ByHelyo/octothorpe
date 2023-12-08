use crate::{
    core::{ApiError, Client},
    rest_error::RestError,
};

pub struct MockClient {}

impl MockClient {
    pub fn new() -> Self {
        MockClient {}
    }
}

impl Client for MockClient {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<url::Url, ApiError<Self::Error>> {
        todo!()
    }

    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, ApiError<Self::Error>> {
        todo!()
    }
}
