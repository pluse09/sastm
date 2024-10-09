use std::fmt;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectUrl(Url);

impl ProjectUrl {
    pub fn new(url: &str) -> Result<Self, String> {
        let parsed_url = Url::parse(url).map_err(|_| "Invalid repository URL".to_string())?;
        Ok(ProjectUrl(parsed_url))
    }

    pub fn value(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for ProjectUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_url_creation_with_normal_value() {
        let url_str = "https://github.com/user/repository";
        let project_url = ProjectUrl::new(url_str);
        assert!(project_url.is_ok());
        assert_eq!(project_url.unwrap().value().as_str(), url_str);
    }

    #[test]
    fn test_project_url_creation_with_invalid_url() {
        let url_str = "not_a_valid_url";
        let project_url = ProjectUrl::new(url_str);
        assert!(project_url.is_err());
    }

    #[test]
    fn test_project_url_display() {
        let url_str = "https://github.com/user/repository";
        let project_url = ProjectUrl::new(url_str).unwrap();
        assert_eq!(format!("{}", project_url), url_str);
    }
}
