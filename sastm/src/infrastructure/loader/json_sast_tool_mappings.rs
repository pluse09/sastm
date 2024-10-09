use crate::domain::model::sast::sast_tool::SastTool;
use crate::domain::model::sast::sast_tool_framework_mapping::SastToolFrameworkMapping;
use crate::domain::model::sast::sast_tool_name::SastToolName;
use crate::domain::model::programming_language::programming_language_name::ProgrammingLanguageName;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonSastToolMappingsProgrammingLanguage {
    pub name: ProgrammingLanguageName,
    pub frameworks: Vec<SastToolFrameworkMapping>,
    pub sast_tool_names: Vec<SastToolName>,
}

#[derive(Deserialize)]
pub struct JsonSastToolMappings {
    pub programming_languages: Vec<JsonSastToolMappingsProgrammingLanguage>,
    pub sast_tools: Vec<SastTool>,
}
