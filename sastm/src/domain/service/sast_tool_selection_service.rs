use std::error::Error;
use serde_json::map;
use tempfile::TempDir;

use crate::domain::model::project::project::Project;
use crate::domain::model::sast;
use crate::domain::model::sast::sast_tool::SastTool;
use crate::domain::provider::sast_tool_mapping::SastToolMapping;

pub struct SastToolSelectionService;

impl SastToolSelectionService {
    pub fn execute(project: &Project, mappings: &SastToolMapping) -> Option<Vec<SastTool>> {
        let programming_languages_name= project.programming_languages;
        let framework_name = &project.framework;

        let mut sast_tools: Vec<SastTool> = Vec::new();

        if let Some(framework_mapping) = mappings.framework_mappings.iter().find(|item| &item.framework_name == framework_name) {
            sast_tools.extend(framework_mapping.get_sast_tools().iter().cloned());
        }
        if let Some(language_mapping) = mappings.language_mappings.iter().find(|item| &item.programming_language_name == programming_languages_name) {
            sast_tools.extend(language_mapping.get_sast_tools().iter().cloned());
        }

        if sast_tools.is_empty() {
            None
        } else {
            Some(sast_tools)
        }
    }

}

// todo: unittest