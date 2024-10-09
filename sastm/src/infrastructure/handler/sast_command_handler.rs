use shaku::{Component, Interface, HasComponent};
use git2::{Cred, RemoteCallbacks, FetchOptions};
use git2::build::RepoBuilder;
use tempfile::TempDir;
use std::error::Error;

use crate::domain::model::sast::sast_tool_command::SastToolCommand;
use crate::domain::handler::sast_command_handler::SastCommandHandler;
use crate::domain::model::application_root_dir::application_root_dir::ApplicationRootDir;

#[derive(Component)]
#[shaku(interface = SastCommandHandler)]
pub struct SastCommandHandlerImpl;

impl SastCommandHandler for SastCommandHandlerImpl {

    fn execute(&self, command: SastToolCommand, application_root_dir: ApplicationRootDir) -> Result<String, Box<dyn Error>> {
        // 
        Err(())
    }

}