use http::{header, HeaderMap, HeaderValue};

use crate::auth_error::AuthResult;

pub enum Auth {
    Token(String),
}

impl Auth {
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        match self {
            Auth::Token(token) => {
                let mut token_header_val = HeaderValue::from_str(token)?;
                token_header_val.set_sensitive(true);
                headers.insert(header::AUTHORIZATION, token_header_val);
            }
        }

        Ok(headers)
    }
}
