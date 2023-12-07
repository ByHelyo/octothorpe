use http::Uri;
use url::Url;

use super::{api_error::ApiError, client::Client};

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

pub trait Query<T, C>
where
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
