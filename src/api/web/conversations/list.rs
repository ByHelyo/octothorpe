use std::borrow::Cow;

use derive_builder::Builder;
use http::Method;

use crate::core::Endpoint;

#[derive(Builder)]
#[builder(setter(strip_option))]
pub struct Conversationslist {}

impl Conversationslist {
    pub fn builder() -> ConversationslistBuilder {
        ConversationslistBuilder::default()
    }
}

impl Endpoint for Conversationslist {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "conversations.list".into()
    }
}