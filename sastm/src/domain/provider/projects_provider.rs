use crate::domain::model::project::project::Project;

pub trait ProjectsProvider {
    fn load(&self) -> Result<Vec<Project>, Box<dyn std::error::Error>>;
}