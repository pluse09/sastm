use shaku::{Component, Interface};
use std::error::Error;
use tempfile::TempDir;


use crate::domain::model::project::project_url::ProjectUrl;

pub trait ProjectRepository: Interface {
    fn fetch_project(&self, url: ProjectUrl) -> Result<TempDir, Box<dyn Error>>;
}