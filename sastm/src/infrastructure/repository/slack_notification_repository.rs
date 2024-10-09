use shaku::{Component, Interface, HasComponent};
use std::error::Error;
use std::env;
use serde_json::json;
use reqwest::Client;
use reqwest::Url as ReqwestUrl;

use crate::domain::model::notification::notification_url::NotificationUrl;
use crate::domain::model::project::project::Project;
use crate::domain::repository::notification_repository::NotificationRepository;
use crate::domain::model::parser::sast_parser::ParsedResult;
use crate::domain::model::channel_id::slack_channel_id::SlackChannelId;

#[derive(Component)]
#[shaku(interface = NotificationRepository)]
pub struct SlackNotificationRepositoryImpl;

impl NotificationRepository for SlackNotificationRepositoryImpl {

    fn notify(&self, project: Project, parsed_results: Vec<ParsedResult>, channel_id: Option<SlackChannelId>) -> Result<(), Box<dyn Error>> {
        
        let channel_id = match channel_id {
            Some(id) => id,
            None => return Err("Channel ID is missing.".into()),
        };

        // todo: PDF形式
        let client = Client::new();
        let message = format!("Project URL: {}, Branch Name: {} \n{}", project.url, project.branch_name, parsed_results.iter()
            .map(|parsed_result| format!("Threshold: {:?}, Category: {}, File: {}, Line: {}", parsed_result.threshold, parsed_result.category, parsed_result.file, parsed_result.line))
            .collect::<Vec<_>>()
            .join("\n"));

        let payload = json!({
            "channel": channel_id.value(),
            "text": format!("Notification from Vuln Code Detector:\n{}", message),
        });

        let response = client
            .post(ReqwestUrl::parse(project.notification_url.value()))
            .bearer_auth(env::var("SLACK_TOKEN")?)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .error_for_status();

        Ok(())

    }

}