use crate::domain::model::sast::sast_tool_framework_mapping::SastToolFrameworkMapping;
use crate::domain::model::sast::sast_tool_programming_language_mapping::SastProgrammingLanguageMapping;

pub struct SastToolMapping {
    pub framework_mappings: Vec<SastToolFrameworkMapping>,
    pub language_mappings: Vec<SastProgrammingLanguageMapping>,
}