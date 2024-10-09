use std::fmt;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SastToolName(String);

impl SastToolName {
    pub fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() {
            return Err("SAST name cannot be empty.".into());
        }
        if name.len() > 20 {
            return Err("SAST name cannot exceed 20 characters.".into());
        }
        Ok(SastToolName(name.into()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SastToolName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sast_tool_name_creation_with_normal_value() {
        let name_str = "Brakeman";
        let name = SastToolName::new(name_str);
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), name_str);
    }

    #[test]
    fn test_sast_tool_name_creation_with_maximum_boundary_value() {
        let name_20 = "A".repeat(20);
        let name = SastToolName::new(&name_20);
        assert!(name.is_ok());
    }

    #[test]
    fn test_sast_tool_name_creation_with_exceeding_boundary_value() {
        let name_21 = "A".repeat(21);
        let name = SastToolName::new(&name_21);
        assert!(name.is_err());
    }

    #[test]
    fn test_sast_tool_name_creation_with_empty_string() {
        let name = SastToolName::new("");
        assert!(name.is_err());
    }

    #[test]
    fn test_sast_tool_name_creation_with_extremely_long_string() {
        let long_name = "A".repeat(100);
        let name = SastToolName::new(&long_name);
        assert!(name.is_err());
    }

    #[test]
    fn test_sast_tool_name_display() {
        let name_str = "Brakeman";
        let name = SastToolName::new(name_str).unwrap();
        assert_eq!(format!("{}", name), name_str);
    }
}
