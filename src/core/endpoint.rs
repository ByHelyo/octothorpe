use std::borrow::Cow;

use http::{Method, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::{
    api_error::ApiError,
    client::Client,
    query::{self, Query},
    query_params::QueryParams,
};

pub trait Endpoint {
    fn method(&self) -> Method;

    fn endpoint(&self) -> Cow<'static, str>;

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));

        let rsp = client.rest(req)?;
        let status = rsp.status();
        let val = if let Ok(val) = serde_json::from_slice::<Value>(rsp.body()) {
            val
        } else {
            return Err(ApiError::server_error(status, rsp.body()));
        };

        if val.get("error").is_some() {
            return Err(ApiError::from_slack(val));
        }

        serde_json::from_value::<T>(val).map_err(ApiError::data_type::<T>)
    }
}
