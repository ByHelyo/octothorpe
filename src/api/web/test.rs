use std::borrow::Cow;

use derive_builder::Builder;
use http::Method;

use crate::core::Endpoint;

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
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "api.test".into()
    }
}
