use url::Url;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationUrl(Url);

impl NotificationUrl {
    pub fn new(url: &str) -> Result<Self, String> {
        let parsed_url = Url::parse(url).map_err(|_| "Invalid notification Url URL".to_string())?;
        Ok(NotificationUrl(parsed_url))
    }

    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for NotificationUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
