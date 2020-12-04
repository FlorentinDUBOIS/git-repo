//! # Command line interface module
//!
//! This module export all stuffs to interact with the command line.
use std::error::Error;
use std::path::PathBuf;
use std::rc::Rc;

use structopt::StructOpt;

use crate::cfg::Configuration;

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
        origin: String,
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
    /// Initialize configuration
    #[structopt(name = "init")]
    Initialize {
        /// Path where initialize the configuration
        #[structopt(name = "path")]
        path: Option<PathBuf>,
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

pub fn add(
    _config: Rc<Configuration>,
    _origin: &str,
    _path: &Option<PathBuf>,
    _name: &Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn remove(_config: Rc<Configuration>, _name: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn pull(
    _config: Rc<Configuration>,
    _name: &Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn status(
    _config: Rc<Configuration>,
    _strip: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn initialize(
    _config: Rc<Configuration>,
    _path: &Option<PathBuf>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn sync(
    _config: Rc<Configuration>,
    _name: &Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}

pub fn for_each(
    _config: Rc<Configuration>,
    _command: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    unimplemented!()
}
