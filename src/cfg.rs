//! # Configuration module
//!
//! This module export all stuffs to serialize, deserialize and interact with the configuration
use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::HashMap;
use std::io::Write;

#[derive(Snafu, Debug)]
pub enum ConfigurationError {
    #[snafu(display("could not load configuration, {}", err))]
    Load { err: Box<dyn Error + Send + Sync> },
    #[snafu(display("failed to load configuration '{}', {}", path.display(), err))]
    ReadFile { err: ConfigError, path: PathBuf },
    #[snafu(display("failed to cast configuration, {}", err))]
    Cast { err: ConfigError },
    #[snafu(display("failed to set default for '{}', {}", name, err))]
    Default {
        name: &'static str,
        err: ConfigError,
    },
    #[snafu(display("failed to create directory '{}', {}", dir.display(), err))]
    CreateDirectory { err: io::Error, dir: PathBuf },
    #[snafu(display("failed to create file '{}', {}", path.display(), err))]
    CreateFile { err: io::Error, path: PathBuf },
    #[snafu(display("failed to serialize configuration, {}", err))]
    Ser { err: toml::ser::Error },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repository {
    #[serde(rename = "path")]
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    #[serde(skip_serializing)]
    path: PathBuf,
    #[serde(rename = "repositories", default = "std::default::Default::default")]
    pub repositories: HashMap<String, Repository>,
}

impl TryFrom<PathBuf> for Configuration {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut config = Config::new();

        config
            .set_default(
                "path",
                format!(
                    "{}/.config/{}/config.toml",
                    env!("HOME"),
                    env!("CARGO_PKG_NAME")
                ),
            )
            .map_err(|err| ConfigurationError::Default { name: "path", err })?;

        config
            .merge(File::from(path.to_owned()).required(true))
            .map_err(|err| ConfigurationError::ReadFile { err, path })?;

        Ok(config
            .try_into()
            .map_err(|err| ConfigurationError::Cast { err })?)
    }
}

impl Configuration {
    pub fn try_new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let name = env!("CARGO_PKG_NAME");
        let mut config = Config::new();

        config
            .set_default(
                "path",
                format!("{}/.config/{}/config.toml", env!("HOME"), name),
            )
            .map_err(|err| ConfigurationError::Default { name: "path", err })?;

        let path = PathBuf::from(format!("/etc/{}/config", name));
        config
            .merge(File::from(path.to_owned()).required(false))
            .map_err(|err| ConfigurationError::ReadFile { err, path })?;

        let path = PathBuf::from(format!("{}/.config/{}/config", env!("HOME"), name));
        config
            .merge(File::from(path.to_owned()).required(false))
            .map_err(|err| ConfigurationError::ReadFile { err, path })?;

        let path = PathBuf::from("config");
        config
            .merge(File::from(path.to_owned()).required(false))
            .map_err(|err| ConfigurationError::ReadFile { err, path })?;

        Ok(config
            .try_into()
            .map_err(|err| ConfigurationError::Cast { err })?)
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<&Repository> {
        self.repositories.get(name)
    }

    #[inline]
    pub fn contains(&self, name: &str) -> bool {
        self.repositories.contains_key(name)
    }

    #[inline]
    pub fn remove(&mut self, name: &str) -> Option<Repository> {
        self.repositories.remove(name)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(path) = self.path.parent() {
            fs::create_dir_all(path.to_owned()).map_err(|err| {
                ConfigurationError::CreateDirectory {
                    dir: path.to_path_buf(),
                    err,
                }
            })?;
        }

        let content = toml::to_string(self).map_err(|err| ConfigurationError::Ser { err })?;

        let mut file = fs::File::create(self.path.to_owned()).map_err(|err| {
            ConfigurationError::CreateFile {
                path: self.path.to_owned(),
                err,
            }
        })?;

        file.write_all(content.as_bytes())
            .map_err(|err| ConfigurationError::CreateFile {
                path: self.path.to_owned(),
                err,
            })?;

        file.sync_all()
            .map_err(|err| ConfigurationError::CreateFile {
                path: self.path.to_owned(),
                err,
            })?;

        Ok(())
    }
}
