//! # Configuration module
//!
//! This module export all stuffs to serialize, deserialize and interact with the configuration
use std::convert::TryFrom;
use std::error::Error;
use std::path::PathBuf;

use config::{Config, File};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    #[serde(rename = "repositories", default = "Default::default")]
    pub repositories: Vec<Repository>,
}

impl TryFrom<PathBuf> for Configuration {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let config = Config::new()
            .merge(File::from(path).required(true))
            .map_err(|err| format!("failed to load configuration, {}", err))?
            .to_owned();

        Ok(config
            .try_into()
            .map_err(|err| format!("failed to cast configuration, {}", err))?)
    }
}

impl Configuration {
    pub fn try_new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let name = env!("CARGO_PKG_NAME");
        let home = env!("HOME");
        let mut config = Config::new();

        config
            .merge(File::from(PathBuf::from(format!("/etc/{}/config", name))).required(false))
            .map_err(|err| {
                format!(
                    "failed to load configuration from '/etc/{}/config', {}",
                    name, err
                )
            })?;

        config
            .merge(File::from(PathBuf::from(format!("{}/.{}", home, name))).required(false))
            .map_err(|err| {
                format!(
                    "failed to load configuration from '{}/.{}', {}",
                    home, name, err
                )
            })?;

        config
            .merge(File::from(PathBuf::from("config")).required(false))
            .map_err(|err| {
                format!(
                    "failed to load configuration from current directory, {}",
                    err
                )
            })?;

        Ok(config
            .try_into()
            .map_err(|err| format!("failed to cast configuration, {}", err))?)
    }
}
