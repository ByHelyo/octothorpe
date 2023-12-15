use std::borrow::Cow;

use derive_builder::Builder;
use http::{Method, StatusCode};
use url::Url;

#[derive(Builder)]
pub struct MockClient {
    #[builder(default = "Method::GET")]
    pub method: Method,
    #[builder(default = "StatusCode::OK")]
    pub status: StatusCode,
    #[builder(default)]
    pub params: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    pub endpoint: &'static str,
}

impl MockClientBuilder {
    pub fn add_query_params(
        &mut self,
        pairs: &[(&'static str, &'static str)],
    ) -> &mut Self {
        self.params
            .get_or_insert_with(Vec::new)
            .extend(pairs.iter().cloned().map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl MockClient {
    pub fn builder() -> MockClientBuilder {
        MockClientBuilder::default()
    }

    pub fn check(&self, method: Method, url: &Url) {
        assert_eq!(method, self.method);

        assert_eq!("https", url.scheme());
        assert_eq!("", url.username());
        assert_eq!(None, url.password());
        assert_eq!("slack.com", url.host_str().unwrap());
        assert_eq!(None, url.port());
        assert_eq!(format!("/api/{}", self.endpoint), url.path());

        assert_eq!(url.fragment(), None);

        let mut count = 0;
        for (ref key, ref value) in url.query_pairs() {
            let found =
                self.params.iter().any(|(expected_key, expected_value)| {
                    key == expected_key && value == expected_value
                });

            if !found {
                panic!("unexpected query parameter `{}={}`", key, value);
            }
            count += 1;
        }
        assert_eq!(count, self.params.len());
        assert_eq!(url.fragment(), None);
    }
}
