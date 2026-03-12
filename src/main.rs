mod cli;
mod core;
mod utils;

use crate::cli::{App, AppCommands};
#[allow(unused_imports)]
use crate::core::{LiferayProject, ProjectType, Workspace};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    let ws = LiferayProject {
        current_dir: std::env::current_dir().unwrap_or_default(),
    };

    match args.command {
        AppCommands::Env { target } => {
            let root = ws.find_root()?;
            let project_type = ws.detect_type(&root);

            println!("Root: {:?}", root);
            println!("Project Type: {:?}", project_type);

            if let Some(version) = ws.get_liferay_version(&root) {
                println!("Liferay Version: {}", version);
            }

            if let Some(t) = target {
                println!("Checking environment for: {}", t);
            }

            // Example using the XML utility
            let _ = utils::find_elements_by_name;
        }
        AppCommands::Data { force } => {
            println!("Data operation initiated (Force={})", force);
        }
    }

    Ok(())
}
