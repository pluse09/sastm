use serde::Deserialize;
use super::sast_tool_name::SastToolName;
use super::sast_tool_command::SastToolCommand;

#[derive(Debug, Clone, Deserialize)]
pub struct SastTool {
    pub name: SastToolName,
    pub command: SastToolCommand,
}

impl SastTool {
    pub fn new(name: SastToolName, command: SastToolCommand) -> Self {
        SastTool { name, command }
    }
}