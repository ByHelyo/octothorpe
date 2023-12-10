use std::borrow::Cow;

use derive_builder::Builder;
use http::Method;

use crate::core::{Endpoint, QueryParams};

#[derive(Builder)]
#[builder(setter(strip_option))]
pub struct Test<'a> {
    #[builder(setter(into), default)]
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

    fn parameters(&self) -> crate::core::QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("error", self.error.as_ref());

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_parameter() {
        Test::builder().build().unwrap();
    }

    #[test]
    fn error_is_optionnal() {
        Test::builder().error("my error").build().unwrap();
    }
}