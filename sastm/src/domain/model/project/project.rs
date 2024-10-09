use super::project_url::ProjectUrl;

use crate::domain::model::programming_language::programming_language_name::ProgrammingLanguageName;
use crate::domain::model::framework::framework_name::FrameworkName;
use crate::domain::model::notification::notification_url::NotificationUrl;
use crate::domain::model::application_root_dir::application_root_dir::ApplicationRootDir;
use crate::domain::model::channel_id::slack_channel_id::SlackChannelId;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub url: ProjectUrl,
    pub branch_name: Option<String>,
    pub application_root_dir: ApplicationRootDir,
    pub programming_languages: Vec<ProgrammingLanguageName>,
    pub framework: Option<FrameworkName>,
    pub threshold: Threshold,
    pub notification_url: NotificationUrl,
    pub channel_id: Option<SlackChannelId>
}

impl Project {
    pub fn new(
        url: ProjectUrl,
        branch_name: Option<String>,
        application_root_dir: ApplicationRootDir,
        programming_languages: Vec<ProgrammingLanguageName>,
        framework: Option<FrameworkName>,
        threshold: Threshold,
        notification_url: NotificationUrl,
        channel_id: Option<SlackChannelId>,
    ) -> Self {
        Project {
            url,
            branch_name.unwrap_or_else(|| "main".to_string()),
            application_root_dir,
            programming_languages,
            framework,
            threshold,
            notification_url,
            channel_id,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// }
