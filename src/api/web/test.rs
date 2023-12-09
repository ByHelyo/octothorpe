use std::borrow::Cow;

use derive_builder::Builder;
use http::Method;

use crate::core::{Endpoint, QueryParams};

#[derive(Builder)]
#[builder(setter(strip_option))]
pub struct Test<'a> {
    #[builder(setter(into))]
    error: Option<Cow<'a, str>>,
}

impl<'a> Test<'a> {
    pub fn builder() -> TestBuilder<'a> {
        TestBuilder::default()
    }
}

impl<'a> Endpoint for Test<'a> {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api.test".into()
    }
}
