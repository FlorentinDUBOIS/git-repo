//! # git-repo
//!
//! Manage multiples git repositories with ease
use std::cell::RefCell;
use std::convert::TryFrom;
use std::error::Error;
use std::rc::Rc;

use slog_scope::crit;

use crate::cfg::{Configuration, ConfigurationError};
use crate::cli::{Args, Command, CommandError};

mod cfg;
mod cli;
mod log;
mod srv;

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _guard = log::initialize(args.verbosity);
    let result = match args.config {
        Some(path) => Configuration::try_from(path).map_err(|err| ConfigurationError::Load { err }),
        None => Configuration::try_new().map_err(|err| ConfigurationError::Load { err }),
    };

    let config = match result {
        Ok(config) => Rc::new(RefCell::new(config)),
        Err(err) => {
            crit!("{}", err);
            return Err(err.into());
        }
    };

    let result = match &args.command {
        Command::Add { origin, path, name } => {
            cli::add(config, origin, path, name).map_err(|err| CommandError::Execution {
                command: "add",
                err,
            })
        }
        Command::Remove { name } => {
            cli::remove(config, name).map_err(|err| CommandError::Execution {
                command: "remove",
                err,
            })
        }
        Command::Pull { name } => cli::pull(config, name).map_err(|err| CommandError::Execution {
            command: "pull",
            err,
        }),
        Command::Status { strip } => {
            cli::status(config, *strip).map_err(|err| CommandError::Execution {
                command: "status",
                err,
            })
        }
        Command::Sync { name } => cli::sync(config, name).map_err(|err| CommandError::Execution {
            command: "sync",
            err,
        }),
        Command::ForEach { command } => {
            cli::for_each(config, command).map_err(|err| CommandError::Execution {
                command: "foreach",
                err,
            })
        }
    };

    if let Err(err) = result {
        crit!("{}", err);
        return Err(err.into());
    }

    Ok(())
}
