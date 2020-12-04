//! # git-repo
//!
//! Manage multiples git repositories with ease
use std::convert::TryFrom;
use std::error::Error;
use std::rc::Rc;

use crate::cfg::Configuration;
use crate::cli::{Args, Command};

mod cfg;
mod cli;
mod log;

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _guard = log::initialize(args.verbosity);
    let config = match args.config {
        Some(path) => Rc::new(
            Configuration::try_from(path)
                .map_err(|err| format!("could not load configuration, {}", err))?,
        ),
        None => Rc::new(
            Configuration::try_new()
                .map_err(|err| format!("could not load configuration, {}", err))?,
        ),
    };

    match &args.command {
        Command::Add { origin, path, name } => cli::add(config, origin, path, name),
        Command::Remove { name } => cli::remove(config, name),
        Command::Pull { name } => cli::pull(config, name),
        Command::Status { strip } => cli::status(config, *strip),
        Command::Initialize { path } => cli::initialize(config, path),
        Command::Sync { name } => cli::sync(config, name),
        Command::ForEach { command } => cli::for_each(config, command),
    }
}
