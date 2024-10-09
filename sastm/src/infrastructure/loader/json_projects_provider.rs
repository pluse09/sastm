use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::domain::model::project::project::Project;
use crate::domain::provider::projects_provider::ProjectsProvider;

pub struct JsonProjectsProvider {
    file_path: String,
}

impl JsonProjectsProvider {
    pub fn new(file_path: String) -> Self {
        JsonProjectsProvider { file_path }
    }
}

impl ProjectsProvider for JsonProjectsProvider {
    fn load(&self) -> Result<Vec<Project>, Box<dyn Error>> {
        let file = File::open(&Path::new(&self.file_path))?;
        let projects: Vec<Project> = serde_json::from_reader(file)?;

        Ok(projects)
    }
}
