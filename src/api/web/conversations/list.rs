use std::borrow::Cow;

use http::Method;

use crate::core::Endpoint;

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

#[derive(Default)]
pub struct ConversationslistBuilder {}

impl ConversationslistBuilder {
    pub fn build(self) -> Conversationslist {
        Conversationslist {}
    }
}
