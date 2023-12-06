use std::borrow::Cow;

use http::Method;
use serde::de::DeserializeOwned;

use super::{
    api_error::ApiError, body_error::BodyError, client::Client, query::Query,
    query_params::QueryParams,
};

trait Endpoint {
    fn method(&self) -> Method;

    fn endpoint(&self) -> Cow<'static, str>;

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        todo!()
    }
}
