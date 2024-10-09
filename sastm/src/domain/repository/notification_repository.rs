use shaku::{Component, Interface};
use std::error::Error;

use crate::domain::model::notification::notification_url::NotificationUrl;
use crate::domain::model::parser::sast_parser::ParsedResult;
use crate::domain::model::channel_id::slack_channel_id::SlackChannelId;
use crate::domain::model::project::project::Project;

pub trait NotificationRepository: Interface {
    fn notify(&self, project: Project, parsed_result: Vec<ParsedResult>, channel_id: Option<SlackChannelId>) -> Result<(), Box<dyn Error>>;
}