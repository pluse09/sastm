use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgrammingLanguageName(String);

impl ProgrammingLanguageName {
    pub fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Language name cannot be empty.".into());
        }
        if name.len() > 20 {
            return Err("SAST Tool name cannot exceed 20 characters.".into());
        }
        Ok(ProgrammingLanguageName(name.into()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProgrammingLanguageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_programming_language_name_creation_with_normal_value() {
        let language_name = ProgrammingLanguageName::new("Ruby");
        assert!(language_name.is_ok());
        assert_eq!(language_name.unwrap().value(), "Ruby");
    }

    #[test]
    fn test_programming_language_name_creation_with_maximum_boundary_value() {
        let name_20 = "A".repeat(20);
        let language_name = ProgrammingLanguageName::new(&name_20);
        assert!(language_name.is_ok());
    }

    #[test]
    fn test_programming_language_name_creation_with_exceeding_boundary_value() {
        let name_21 = "A".repeat(21);
        let language_name = ProgrammingLanguageName::new(&name_21);
        assert!(language_name.is_err());
    }

    #[test]
    fn test_programming_language_name_creation_with_empty_string() {
        let language_name = ProgrammingLanguageName::new("");
        assert!(language_name.is_err());
    }

    #[test]
    fn test_programming_language_name_creation_with_extremely_long_string() {
        let long_name = "A".repeat(100);
        let language_name = ProgrammingLanguageName::new(&long_name);
        assert!(language_name.is_err());
    }

    #[test]
    fn test_programming_language_name_display() {
        let language_name = ProgrammingLanguageName::new("Ruby").unwrap();
        assert_eq!(format!("{}", language_name), "Ruby");
    }
}
