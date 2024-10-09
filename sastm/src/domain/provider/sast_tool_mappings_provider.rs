use super::sast_tool_mapping::SastToolMapping;

pub trait SastToolMappingsProvider {
    fn load(&self) -> Result<SastToolMapping, Box<dyn std::error::Error>>;
}