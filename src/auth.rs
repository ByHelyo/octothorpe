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
                let token = "Bearer ".to_string() + token;
                let mut token_header_val = HeaderValue::from_str(&token)?;
                token_header_val.set_sensitive(true);
                headers.insert(header::AUTHORIZATION, token_header_val);
            }
        }

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth_error::AuthError;

    #[test]
    fn invalid_header() {
        let auth = Auth::Token("\n".to_string());
        let mut headers = HeaderMap::new();
        let headers = auth.set_header(&mut headers).unwrap_err();

        dbg!(&headers);
        print!("{}", headers);
        assert!(matches!(headers, AuthError::HeaderValue { .. }));
    }

    #[test]
    fn valid_header() {
        let auth = Auth::Token("mytoken".to_string());
        let mut headers = HeaderMap::new();
        let headers = auth.set_header(&mut headers).unwrap();

        assert_eq!(headers.get(header::AUTHORIZATION).unwrap(), "Bearer mytoken");
    }
}
