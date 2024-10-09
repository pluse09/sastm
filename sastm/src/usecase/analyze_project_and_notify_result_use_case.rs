use std::error::Error;
use std::sync::Arc;

use crate::domain::handler::sast_command_handler::SastCommandHandler;
use crate::domain::model::parser::sast_parser::ParsedResult;
use crate::domain::model::sast::sast_tool::SastTool;
use crate::domain::provider::projects_provider::ProjectsProvider;
use crate::domain::provider::sast_tool_mappings_provider::SastToolMappingsProvider;
use crate::domain::repository::notification_repository::NotificationRepository;
use crate::domain::repository::project_repository::ProjectRepository;
use crate::domain::service::sast_tool_selection_service::SastToolSelectionService;
use crate::infrastructure::selector::sast_parser_selector::SastParserSelector;

pub struct AnalyzeProjectAndNotifyResultUseCase {
    project_repository: Arc<dyn ProjectRepository>,
    notification_repository: Arc<dyn NotificationRepository>,
    sast_command_handler: Arc<dyn SastCommandHandler>,
    projects_provider: Arc<dyn ProjectsProvider>,
    sast_tool_mappings_provider: Arc<dyn SastToolMappingsProvider>
}

impl AnalyzeProjectAndNotifyResultUseCase {
    pub fn new(
        project_repository: Arc<dyn ProjectRepository>,
        notification_repository: Arc<dyn NotificationRepository>,
        sast_command_handler: Arc<dyn SastCommandHandler>,
        projects_provider: Arc<dyn ProjectsProvider>,
        sast_tool_mappings_provider: Arc<dyn SastToolMappingsProvider>,
    ) -> Self {
        Self {
            project_repository,
            notification_repository,
            sast_command_handler,
            projects_provider,
            sast_tool_mappings_provider
        }
    }
    
    pub fn execute(&self) {
        // todo: 分割
        let projects = self.projects_provider.load()?;
        let tool_mappings = self.sast_tool_mappings_provider.load()?;

        // todo: 非同期
        for project in projects.into() {
            let temp_dir = match self.project_repository.fetch_project(project.url) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Failed to fetch project '{}': {}", project.url, e);
                    continue;
                }
            };

            let sast_tools: Option<Vec<SastTool>> = SastToolSelectionService::execute(project, &tool_mappings);
            let mut results: Vec<ParsedResult> = Vec::new();

            if let Some(sast_tools) = sast_tools {
                for sast_tool in sast_tools.iter() {
                    let output = match self.sast_command_handler.execute(sast_tool.command, project.application_root_dir) {
                        Ok(o) => o,
                        Err(e) => {
                            eprintln!("Failed to execute SAST tool '{}': {}", sast_tool.name, e);
                            continue;
                        }
                    };

                    // Factoryパターンを使った方が良いらしい。らしい。
                    match SastParserSelector::execute(sast_tool.name) {
                        Some(p) => {
                            let result = p.parse(output);
                            results.push(result);
                        },
                        None => {
                            // todo: Parser対応してない場合
                        }
                    };
                }
                // Slack以外の通知先にも対応する場合、URLから通知先を選択するNotifierSelectorみたいなのを作ってここで使う。（Factoryパターン活用？？）
                match self.notification_repository.notify(project, results, project.channel_id) {
                    Ok(r) => {},
                    Err(e) => {
                        eprintln!("Failed to notify: {}", e);
                        continue;
                    }
                }

            } else {
                println!("プロジェクト{}に使われてるプログラミング言語またはフレームワークに対応するSASTツールが設定されていません。", project.url);
            }
        }
    }

}