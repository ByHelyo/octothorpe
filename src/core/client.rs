use std::error::Error;

use bytes::Bytes;
use http::{request::Builder, Response};
use url::Url;

use super::api_error::ApiError;

pub trait Client {
    type Error: Error + Send + Sync + 'static;

    fn rest_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<Url, ApiError<Self::Error>>;

    fn rest(
        &self,
        request: Builder,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
