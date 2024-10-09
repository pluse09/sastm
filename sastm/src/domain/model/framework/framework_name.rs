use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkName(String);

impl FrameworkName {
    pub fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Framework name cannot be empty.".into());
        }
        if name.len() > 20 {
            return Err("SAST Tool name cannot exceed 20 characters.".into());
        }
        Ok(FrameworkName(name.into()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FrameworkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_name_creation_with_normal_value() {
        let framework_name = FrameworkName::new("Rails");
        assert!(framework_name.is_ok());
        assert_eq!(framework_name.unwrap().value(), "Rails");
    }

    #[test]
    fn test_framework_name_creation_with_maximum_boundary_value() {
        let name_20 = "A".repeat(20);
        let framework_name = FrameworkName::new(&name_20);
        assert!(framework_name.is_ok());
    }

    #[test]
    fn test_framework_name_creation_with_exceeding_boundary_value() {
        let name_21 = "A".repeat(21);
        let framework_name = FrameworkName::new(&name_21);
        assert!(framework_name.is_err());
    }

    #[test]
    fn test_framework_name_creation_with_empty_string() {
        let framework_name = FrameworkName::new("");
        assert!(framework_name.is_err());
    }

    #[test]
    fn test_framework_name_creation_with_extremely_long_string() {
        let long_name = "A".repeat(100);
        let framework_name = FrameworkName::new(&long_name);
        assert!(framework_name.is_err());
    }
}
