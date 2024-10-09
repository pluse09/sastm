use shaku::{Component, Interface};
use std::error::Error;

use crate::domain::model::sast::sast_tool_command::SastToolCommand;
use crate::domain::model::application_root_dir::application_root_dir::ApplicationRootDir;

pub trait SastCommandHandler: Interface {
    fn execute(&self, command: SastToolCommand, application_root_dir: ApplicationRootDir) -> Result<String, Box<dyn Error>>;
}