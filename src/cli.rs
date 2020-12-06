//! # Command line interface module
//!
//! This module export all stuffs to interact with the command line.
use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::fs::canonicalize;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;

use snafu::Snafu;
use structopt::StructOpt;

use crate::cfg::{Configuration, Repository};
use crate::srv::git;

#[derive(StructOpt, Clone, Debug)]
pub struct Args {
    /// Increase log verbosity
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: usize,
    /// Specify a configuration file
    #[structopt(short = "c", long = "config")]
    pub config: Option<PathBuf>,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Clone, Debug)]
pub enum Command {
    /// Add a git repository to manage
    #[structopt(name = "add")]
    Add {
        /// Git url to clone the repository
        #[structopt(name = "origin")]
        origin: PathBuf,
        /// Path where clone the repository
        #[structopt(name = "path")]
        path: Option<PathBuf>,
        /// Name of the repository
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
    },
    /// Remove a git repository to manage
    #[structopt(name = "remove", aliases = &["rm"])]
    Remove {
        /// Name of the git repository
        #[structopt(name = "name")]
        name: String,
    },
    /// Pull modifications from remote git repositories
    #[structopt(name = "pull")]
    Pull {
        /// Name of the git repository
        #[structopt(name = "name")]
        name: Option<String>,
    },
    /// Retrieve status of git repository
    #[structopt(name = "status")]
    Status {
        /// Strip the output
        #[structopt(short = "s", long = "strip")]
        strip: bool,
    },
    /// Sync modifications between local and remote repository
    #[structopt(name = "sync")]
    Sync {
        /// Name of the git repository
        #[structopt(name = "name")]
        name: Option<String>,
    },
    /// Execute a command in each git repository
    #[structopt(name = "foreach")]
    ForEach {
        /// Command to execute
        #[structopt(name = "command")]
        command: String,
    },
}

#[derive(Snafu, Debug)]
pub enum CommandError {
    #[snafu(display("could not execute command '{}', {}", command, err))]
    Execution {
        command: &'static str,
        err: Box<dyn Error + Send + Sync>,
    },
    #[snafu(display("name '{}' is already taken for this repository", name))]
    NameAlreadyExists { name: String },
    #[snafu(display("path '{}' is already taken for this repository", path.display()))]
    PathAlreadyExists { path: PathBuf },
    #[snafu(display("failed to save configuration, {}", err))]
    SaveConfiguration { err: Box<dyn Error + Send + Sync> },
    #[snafu(display("failed to resolve path of '{}', {}", path.display(), err))]
    ResolvePath { err: io::Error, path: PathBuf },
    #[snafu(display("failed to delete git repository '{}', {}", path.display(), err))]
    DeletePath { err: io::Error, path: PathBuf },
}

pub fn add(
    config: Rc<RefCell<Configuration>>,
    origin: &PathBuf,
    path: &Option<PathBuf>,
    name: &Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Retrieve name of the repository from the origin or the given name
    let name = git::extract_name(origin, name).ok_or_else(|| "failed to retrieve name")?;
    if config.borrow().contains(&name) {
        return Err(CommandError::NameAlreadyExists { name }.into());
    }

    // Compute the path from the argument or from name
    let path = match path {
        Some(path) => path.to_owned(),
        None => PathBuf::from(name.to_owned()),
    };

    // Before cloning check if the repository do not already exists
    if config
        .borrow()
        .contains(&String::from(path.to_string_lossy()))
    {
        return Err(CommandError::PathAlreadyExists { path }.into());
    }

    // Clone the repository
    git::clone(origin, &path)?;

    let path =
        canonicalize(path.to_owned()).map_err(|err| CommandError::ResolvePath { path, err })?;

    config
        .borrow_mut()
        .repositories
        .insert(name.to_string(), Repository { path });

    config
        .borrow()
        .save()
        .map_err(|err| CommandError::SaveConfiguration { err })?;

    Ok(())
}

pub fn remove(
    config: Rc<RefCell<Configuration>>,
    name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut config = config.borrow_mut();
    if let Some(repo) = config.get(name) {
        fs::remove_dir_all(&repo.path).map_err(|err| CommandError::DeletePath {
            path: repo.path.to_owned(),
            err,
        })?;

        config.remove(name);
        config
            .save()
            .map_err(|err| CommandError::SaveConfiguration { err })?;
    }

    Ok(())
}

pub fn pull(
    _config: Rc<RefCell<Configuration>>,
    _name: &Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn status(
    _config: Rc<RefCell<Configuration>>,
    _strip: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn sync(
    _config: Rc<RefCell<Configuration>>,
    _name: &Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn for_each(
    _config: Rc<RefCell<Configuration>>,
    _command: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}
