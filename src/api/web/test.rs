use http::Method;

use crate::core::Endpoint;

pub struct Test {}

impl Test {
    pub fn builder() -> TestBuilder {
        TestBuilder::default()
    }
}

impl Endpoint for Test {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "api.test".to_string().into()
    }
}

#[derive(Default)]
pub struct TestBuilder {}

impl TestBuilder {
    pub fn build(self) -> Test {
        Test {}
    }
}
