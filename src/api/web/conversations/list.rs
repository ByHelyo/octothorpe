use http::Method;

use crate::core::Endpoint;

pub struct Conversationslist {}

impl Conversationslist {
    pub fn builder() -> ConversationslistBuilder {
        ConversationslistBuilder::default()
    }
}

impl Endpoint for Conversationslist {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "conversations.list".to_string().into()
    }
}

#[derive(Default)]
pub struct ConversationslistBuilder {}

impl ConversationslistBuilder {
    pub fn build(self) -> Conversationslist {
        Conversationslist {}
    }
}
