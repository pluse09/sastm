use std::fmt;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlackChannelId(String);

impl SlackChannelId {
    // チャンネルIDの命名規則: 英数字とアンダースコアのみ、最大21文字
    const MAX_LENGTH: usize = 21;

    // todo: エラーケースをStringではなくする
    pub fn new(channel_id: &str) -> Result<Self, String> {
        let valid_id_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

        if channel_id.is_empty() {
            return Err("Slack channel ID cannot be empty.".into());
        }

        if channel_id.len() > Self::MAX_LENGTH {
            return Err(format!("Slack channel ID cannot exceed {} characters.", Self::MAX_LENGTH));
        }

        if !valid_id_regex.is_match(channel_id) {
            return Err("Slack channel ID contains invalid characters. Only alphanumeric characters and underscores are allowed.".into());
        }

        Ok(SlackChannelId(channel_id.into()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

}

impl fmt::Display for SlackChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slack_channel_id_creation_valid() {
        let channel_id_str = "hoge_foo_bar_123";
        let channel_id = SlackChannelId::new(channel_id_str);
        assert!(channel_id.is_ok());
        assert_eq!(channel_id.unwrap().value(), channel_id_str);
    }

    #[test]
    fn test_slack_channel_id_creation_empty() {
        let channel_id = SlackChannelId::new("");
        assert!(channel_id.is_err());
    }

    #[test]
    fn test_slack_channel_id_creation_too_long() {
        let channel_id_str = "hoge_foo_bar_123456789";
        let channel_id = SlackChannelId::new(channel_id_str);
        assert!(channel_id.is_err());
    }

    #[test]
    fn test_slack_channel_id_creation_invalid_chars() {
        let channel_id_str = "hoge!foo$barbar";
        let channel_id = SlackChannelId::new(channel_id_str);
        assert!(channel_id.is_err());
    }
}
