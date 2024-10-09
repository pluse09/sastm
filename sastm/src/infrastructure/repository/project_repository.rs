use git2::{Cred, RemoteCallbacks, FetchOptions};
use git2::build::RepoBuilder;
use shaku::{Component, Interface, HasComponent};
use std::error::Error;
use tempfile::TempDir;

use crate::domain::model::project::project_url::ProjectUrl;
use crate::domain::repository::project_repository::ProjectRepository;

#[derive(Component)]
#[shaku(interface = ProjectRepository)]
pub struct ProjectRepositoryImpl;

impl ProjectRepository for ProjectRepositoryImpl {

    fn fetch_project(&self, url: ProjectUrl) -> Result<TempDir, Box<dyn Error>> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username, _allowed_type| {
            // Use SSH agent for authentication
            Cred::ssh_key_from_agent(username.unwrap_or("git"))
        });
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut repo_builder = RepoBuilder::new();
        repo_builder.fetch_options(fetch_options);

        let temp_dir = tempfile::tempdir()?;
        // let clone_path = temp_dir.path().join("repo");

        repo_builder.clone(url.value(), &temp_dir.path())
            .map(|_| temp_dir)
            .map_err(|err| Box::new(err))
    }

}
