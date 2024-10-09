use crate::domain::model::framework::framework_name::FrameworkName;
use crate::domain::model::sast::sast_tool::SastTool;

pub struct SastToolFrameworkMapping {
    pub framework_name: FrameworkName,
    pub sast_tools: Vec<SastTool>,
}

impl SastToolFrameworkMapping {
    pub fn new(
        framework_name: FrameworkName,
        sast_tools: Vec<SastTool>,
    ) -> Self {
        SastToolFrameworkMapping {
            framework_name,
            sast_tools,
        }
    }

    pub fn get_sast_tools(&self) -> &Vec<SastTool> {
        &self.sast_tools
    }
}