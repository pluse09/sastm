use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SastToolCommand(String);

impl SastToolCommand {
    pub fn new(command: &str) -> Result<Self, String> {
        if command.is_empty() {
            return Err("SAST Tool command cannot be empty.".into());
        }
        Ok(SastToolCommand(command.into()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sast_tool_command_creation_with_normal_value() {
        let command_str = "brakeman -A -f json -q ./path/";
        let command = SastToolCommand::new(command_str);
        assert!(command.is_ok());
    }

    #[test]
    fn test_sast_tool_command_creation_with_empty_string() {
        let command = SastToolCommand::new("");
        assert!(command.is_err());
    }
}