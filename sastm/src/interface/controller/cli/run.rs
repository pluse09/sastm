use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use clap::Args;
use shaku::HasComponent;

use crate::domain::handler::sast_command_handler::SastCommandHandler;
use crate::domain::provider::projects_provider::ProjectsProvider;
use crate::domain::provider::sast_tool_mappings_provider::SastToolMappingsProvider;
use crate::domain::repository::project_repository::ProjectRepository;
use crate::domain::repository::notification_repository::NotificationRepository;
use crate::infrastructure::app_module::ImplModule;
use crate::infrastructure::app_module::MockModule;
use crate::infrastructure::loader::json_projects_provider::JsonProjectsProvider;
use crate::infrastructure::loader::json_sast_tool_mappings_provider::JsonSastToolMappingsProvider;
use crate::usecase::analyze_project_and_notify_result_use_case::AnalyzeProjectAndNotifyResultUseCase;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(long, requires_all = &["frequency", "time"], conflicts_with = "time")]
    pub once: bool,

    /// Set the time to execute the task (format: HH:MM:SS)
    #[arg(long, requires = "frequency", conflicts_with = "once")]
    pub time: Option<String>,

    /// Set the frequency of the task (daily or weekly)
    #[arg(long, value_parser = ["daily", "weekly"], conflicts_with = "once")]
    pub frequency: Option<String>,
}

pub struct Run;

impl Run {
    pub fn execute(args: &RunArgs) {
        if args.once {
            // DIP違反+interface層でinfra層扱ってるが一旦このまま（cli/root.rsから修正しないと）。
            let module                                               = MockModule::builder().build();
            let project_repository: Arc<dyn ProjectRepository>                   = module.resolve();
            let notification_repository: Arc<dyn NotificationRepository>         = module.resolve();
            let sast_command_handler: Arc<dyn SastCommandHandler>                = module.resolve();
            let project_root                                             = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let json_projects_file_path                                 = Path::new(&project_root).join("src/config/projects.json");
            let json_projects_provider                = Arc::new(JsonProjectsProvider::new(json_projects_file_path)) as Arc<dyn ProjectsProvider>;
            let sast_tool_mappings_file_path                            = Path::new(&project_root).join("src/config/sast_tool_mappings.json");
            let sast_tool_mappings_provider   = Arc::new(JsonSastToolMappingsProvider::new(sast_tool_mappings_file_path)) as Arc<dyn SastToolMappingsProvider>;
            let analyze_project_use_case = AnalyzeProjectAndNotifyResultUseCase::new(project_repository, notification_repository, sast_command_handler, json_projects_provider, sast_tool_mappings_provider);
            analyze_project_use_case.execute();
            return;
        }
        eprintln!("No valid options provided. Please specify either --once or both --time and --frequency.");
    }
}

fn is_valid_time_format(time: &str) -> bool {
    time.split(':').count() == 3 && time.split(':').all(|part| part.len() == 2 && u8::from_str(part).is_ok())
}

// #[cfg(test)]
// mod tests {
//     use crate::infrastructure::app_module::MockModule;
//     use super::*;

//     #[test]
//     ...
// }