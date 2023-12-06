use reqwest::blocking;
use url::Url;

use crate::auth::Auth;

struct Slack {
    client: blocking::Client,
    base_url: Url,
    auth: Auth,
}

impl Slack {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        let base_url = Url::parse("https://slack.com/api/").unwrap();

        let client = blocking::Client::new();
        let auth = Auth::Token(token.into());

        Self {
            client,
            base_url,
            auth,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_slack_client() {
        let client = Slack::new("mytoken");
        assert_eq!(
            client.base_url,
            Url::parse("https://slack.com/api/").unwrap()
        );
    }
}
