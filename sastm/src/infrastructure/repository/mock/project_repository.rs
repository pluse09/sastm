use shaku::{Component, Interface};
use tempfile::TempDir;
use std::error::Error;
use std::path::PathBuf;
use std::thread::Builder;
use std::fs;

use crate::domain::model::project::project_url::ProjectUrl;
use crate::domain::repository::project_repository::ProjectRepository;

#[derive(Component)]
#[shaku(interface = ProjectRepository)]
pub struct ProjectRepositoryMock;

impl ProjectRepository for ProjectRepositoryMock {

    fn fetch_project(&self, url: ProjectUrl) -> Result<TempDir, Box<dyn Error>> {

        let project_root = std::env::var("CARGO_MANIFEST_DIR")?;
        let mock_repo_path = PathBuf::from(format!("{}/src/infrastructure/repository/mock/temp_dir", project_root));

        let temp_dir = Builder::new().prefix("mock_tempdir").tempdir_in(".")?;
        let dest_path = temp_dir.path().to_path_buf();

        fs::copy(&mock_repo_path, dest_path.join("repo"))?;

        Ok(temp_dir)

    }

}