use http::Request;
use serde_json::Value;

use super::{Client, Endpoint, Query};
use crate::core::{query, ApiError};

pub struct Ignore<E>
where
    E: Endpoint,
{
    endpoint: E,
}

pub fn ignore<E: Endpoint>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}

impl<E, C> Query<(), C> for Ignore<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(
        &self,
        client: &C,
    ) -> Result<(), super::ApiError<<C as super::Client>::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.endpoint.method())
            .uri(query::url_to_http_uri(url));

        let rsp = client.rest(req)?;
        let val = if let Ok(val) = serde_json::from_slice::<Value>(rsp.body()) {
            val
        } else {
            return Err(ApiError::server_error(rsp.status(), rsp.body()));
        };

        if val.get("error").is_some() {
            return Err(ApiError::from_slack(val));
        }

        Ok(())
    }
}
