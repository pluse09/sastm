use crate::domain::model::programming_language::programming_language_name::ProgrammingLanguageName;
use crate::domain::model::sast::sast_tool::SastTool;

pub struct SastProgrammingLanguageMapping {
    pub programming_language_name: ProgrammingLanguageName,
    pub sast_tools: Vec<SastTool>,
}

impl SastProgrammingLanguageMapping {
    pub fn new(
        programming_language_name: ProgrammingLanguageName,
        sast_tools: Vec<SastTool>,
    ) -> Self {
        SastProgrammingLanguageMapping {
            programming_language_name,
            sast_tools,
        }
    }

    pub fn get_sast_tools(&self) -> &Vec<SastTool> {
        &self.sast_tools
    }
}