use http::{request::Builder, Response};
use url::Url;

use crate::{
    core::{ApiError, Client},
    rest_error::RestError,
};

pub struct MockClient {
    body: Vec<u8>,
}

impl MockClient {
    pub fn new_raw(body: Vec<u8>) -> Self {
        MockClient { body }
    }
}

impl Client for MockClient {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Url, ApiError<Self::Error>> {
        todo!()
    }

    fn rest(
        &self,
        _request: Builder,
        _body: Vec<u8>,
    ) -> Result<Response<bytes::Bytes>, ApiError<Self::Error>> {
        let rsp = Response::builder().body(self.body.into()).unwrap();
        Ok(rsp)
    }
}
