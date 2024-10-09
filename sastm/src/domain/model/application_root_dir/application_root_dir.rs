use std::fmt;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationRootDir(String);

impl ApplicationRootDir {

    pub fn new(dir: &str) -> Result<Self, String> {
        let re = Regex::new(r"^(|\/(?:[\w\-]+\/?)*)$").unwrap();

        if re.is_match(dir) {
            let dir = match dir {
                "/" => "".to_string(),
                d if d.starts_with('/') => format!(".{}", d),
                _ => dir.to_string(),
            };
            Ok(ApplicationRootDir(dir))
        } else {
            Err(format!("Invalid directory format: {}", dir))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
    
}

impl fmt::Display for ApplicationRootDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_root_dir_creation_with_normal_value() {
        let dir = ApplicationRootDir::new("./foo/bar");
        assert!(dir.is_ok());
        assert_eq!(dir.unwrap().value(), "./foo/bar");
    }

    #[test]
    fn test_application_root_dir_creation_with_top_directory() {
        let dir = ApplicationRootDir::new("");
        assert!(dir.is_ok());
        assert_eq!(dir.unwrap().value(), "");
    }

    #[test]
    fn test_application_root_dir_with_root() {
        let dir = ApplicationRootDir::new("/").unwrap();
        assert_eq!(dir.value(), "");
    }

    #[test]
    fn test_application_root_dir_creation_with_dot_slash() {
        let dir = ApplicationRootDir::new("./");
        assert!(dir.is_ok());
        assert_eq!(dir.unwrap().value(), "./");
    }

    #[test]
    fn test_application_root_dir_with_subdir() {
        let dir = ApplicationRootDir::new("/hoge").unwrap();
        assert_eq!(dir.value(), "./hoge");
    }

    #[test]
    fn test_application_root_dir_creation_with_invalid_dir() {
        let dir = ApplicationRootDir::new("foo/bar");
        assert!(dir.is_err());
    }

}
