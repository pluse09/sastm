use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::domain::model::sast::sast_tool::SastTool;
use crate::domain::model::sast::sast_tool_name::SastToolName;
use crate::domain::model::sast::sast_tool_command::SastToolCommand;
use crate::domain::model::framework::framework_name::FrameworkName;
use crate::domain::model::sast::sast_tool_framework_mapping::SastToolFrameworkMapping;
use crate::domain::model::sast::sast_tool_programming_language_mapping::SastProgrammingLanguageMapping;
use crate::domain::model::programming_language::programming_language_name::ProgrammingLanguageName;
use crate::domain::provider::sast_tool_mappings_provider::SastToolMappingsProvider;
use crate::domain::provider::sast_tool_mapping::SastToolMapping;

use super::json_sast_tool_mappings::JsonSastToolMappings;

pub struct JsonSastToolMappingsProvider {
    // todo: String -> JsonFilePath
    file_path: String,
}

impl JsonSastToolMappingsProvider {
    pub fn new(file_path: String) -> Self {
        JsonSastToolMappingsProvider { file_path }
    }
}

impl SastToolMappingsProvider for JsonSastToolMappingsProvider {

    // todo: 分割
    fn load(&self) -> Result<SastToolMapping, Box<dyn Error>> {
        let file = File::open(&Path::new(&self.file_path))?;
        let json_mappings: JsonSastToolMappings = serde_json::from_reader(file)?;

        let mut sast_tool_framework_mapping: Vec<SastToolFrameworkMapping> = Vec::new();
        let programming_languages = json_mappings.programming_languages;

        for p in &programming_languages {
            for f in &p.frameworks {
                let sast_tools = f.sast_tools.iter()
                .map(|sast_tool| {
                    let name = SastToolName::new(&sast_tool.name.value())?;
                    let command = SastToolCommand::new(&sast_tool.command.value())?;
                    Ok::<SastTool, Box<dyn Error>>(SastTool::new(name, command))
                })
                .collect::<Result<Vec<SastTool>, _>>()?;

            let framework_name = FrameworkName::new(&f.framework_name.value())?;
            let framework_mapping = SastToolFrameworkMapping::new(framework_name, sast_tools);
            sast_tool_framework_mapping.push(framework_mapping);
            }
        }

        let mut language_mappings: Vec<SastProgrammingLanguageMapping> = Vec::new();

        for p in &programming_languages {
            let programming_language_name: ProgrammingLanguageName = p.name.clone();
            let sast_tool_names = p.sast_tool_names.iter()
                .map(|sast_tool_name| {
                    json_mappings.sast_tools.iter()
                        .find(|sast_tool| &sast_tool.name == sast_tool_name)
                        .cloned()
                        .ok_or_else(|| format!("SAST tool '{}' not found", sast_tool_name.value()))
                })
                .collect::<Result<Vec<SastTool>, _>>()?;
            language_mappings.push(SastProgrammingLanguageMapping {
                programming_language_name,
                sast_tools: sast_tool_names,
            });
        }

        Ok(SastToolMapping {
            framework_mappings: sast_tool_framework_mapping,
            language_mappings: language_mappings,
        })
    }
}
