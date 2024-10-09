use shaku::{Component, Interface};
use std::error::Error;
use tempfile::TempDir;

use crate::domain::model::notification::notification_url::NotificationUrl;
use crate::domain::model::project::project::Project;
use crate::domain::repository::notification_repository::NotificationRepository;
use crate::domain::model::parser::sast_parser::ParsedResult;
use crate::domain::model::channel_id::slack_channel_id::SlackChannelId;

#[derive(Component)]
#[shaku(interface = NotificationRepository)]
pub struct SlackNotificationRepositoryMock;

impl NotificationRepository for SlackNotificationRepositoryMock {

    fn notify(&self, project: Project, parsed_result: Vec<ParsedResult>, channel_id: Option<SlackChannelId>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

}