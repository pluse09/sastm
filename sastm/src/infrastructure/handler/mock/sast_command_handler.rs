use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use shaku::{Component, Interface};

use crate::domain::model::sast::sast_tool_command::SastToolCommand;
use crate::domain::handler::sast_command_handler::SastCommandHandler;
use crate::domain::model::application_root_dir::application_root_dir::ApplicationRootDir;

#[derive(Component)]
#[shaku(interface = SastCommandHandler)]
pub struct SastCommandHandlerMock;

impl SastCommandHandler for SastCommandHandlerMock {

    fn execute(&self, command: SastToolCommand, application_root_dir: ApplicationRootDir) -> Result<String, Box<dyn Error>> {
        let project_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mock_file_path = Path::new(&project_root).join("src/infrastructure/handler/mock/result.txt");
        let mut file = File::open(mock_file_path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

}