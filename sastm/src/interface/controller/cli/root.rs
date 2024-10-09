use clap::{Parser, Subcommand};
use super::run::{RunArgs, Run};
use super::version::Version;

#[derive(Parser, Debug)]
#[command(name = "sastm")]
#[command(about = "enter about this tool", long_about = None)]
pub struct Root {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(RunArgs),
    Version,
}

impl Root {
    pub fn execute(&self) {
        match &self.command {
            Commands::Run(args) => {
                // let analyze_project_use_case = Box::new(AnalyzeProjectUseCase::new());
                // let run_instance = Run::new(analyze_project_use_case);
                // run_instance.execute(args);
                Run::execute(args)
            },
            Commands::Version => Version::print(),
        }
    }
}