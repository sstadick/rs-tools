use enum_dispatch::enum_dispatch;
use std::env;
use std::fs::{copy, read_dir};
use std::path::Path;
use std::process::Command;
use std::{error::Error, path::PathBuf};

type DynError = Box<dyn Error>;

use clap::Parser;

#[enum_dispatch]
trait CommandImpl {
    fn main(&self) -> Result<(), DynError>;
}

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Install(Install),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}

// -------------- Tasks -----------

#[derive(Parser, Debug)]
struct Install {
    /// The directory to install binaries to
    #[clap(long, short)]
    target_dir: PathBuf,
    /// The projects to install
    #[clap(long, short)]
    projects: Vec<PathBuf>,
}

impl CommandImpl for Install {
    fn main(&self) -> Result<(), DynError> {
        eprintln!("Install stuff to {:?}", self.target_dir);
        let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        for project in &self.projects {
            let status = Command::new(&cargo)
                .current_dir(project_root())
                .args(&[
                    "install",
                    "--path",
                    project.to_str().unwrap(),
                    "--root",
                    project_root().to_str().unwrap(),
                ])
                .status()?;
            if !status.success() {
                return Err("cargo install failed".into());
            }
        }

        // Now move each thing in <project>/bin to "target"
        for entry in read_dir(project_root().join("bin"))? {
            let path = entry?.path();
            if path.is_file() {
                let filename = path.file_name().expect("Can't get name of exe");
                copy(&path, self.target_dir.join(&filename))?;
            }
        }

        Ok(())
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}
