mod domain;
mod interface;
mod infrastructure;
mod usecase;

use clap::Parser;
use crate::interface::controller::cli::root;

// #[tokio::main]
fn main() {
    let cli_root = root::Root::parse();
    cli_root.execute();
}