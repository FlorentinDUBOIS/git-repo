//! # Git module
//!
//! This module provide git functions, methods and structures.
use std::error::Error;
use std::path::PathBuf;

use git2::build::RepoBuilder;
use git2::{
    AutotagOption, Config, Cred, CredentialHelper, FetchOptions, RemoteCallbacks, Repository,
};
use snafu::Snafu;

use crate::cfg;

#[derive(Snafu, Debug)]
pub enum GitError {
    #[snafu(display(
        "failed to clone repository from '{}' into '{}', {}",
        origin.display(),
        path.display(),
        err
    ))]
    Clone {
        origin: PathBuf,
        path: PathBuf,
        err: git2::Error,
    },
    #[snafu(display(
        "failed to pull repository '{}', {}",
        path.display(),
        err
    ))]
    Pull { path: PathBuf, err: git2::Error },
}

pub fn extract_name(origin: &PathBuf, name: &Option<String>) -> Option<String> {
    if name.is_some() {
        return name.to_owned();
    }

    origin
        .file_name()
        .map(|os_str| String::from(os_str.to_string_lossy().replace(".git", "")))
}

// build_fetch_options set a callback on fetch operations which allow to set up
// credentials.
//
// See:
// - https://github.com/rust-lang/git2-rs/issues/394
// - https://github.com/rust-lang/git2-rs/issues/41
// - https://github.com/rust-lang/cargo/blob/bb28e71202260180ecff658cd0fa0c7ba86d0296/src/cargo/sources/git/utils.rs#L344-L391
fn build_fetch_options<'callback>() -> FetchOptions<'callback> {
    let mut callback = RemoteCallbacks::new();

    callback.credentials(|url, username, kind| {
        let config = Config::open_default()?;
        let mut helper = CredentialHelper::new(url);
        helper.config(&config);

        if kind.is_ssh_key() {
            Cred::ssh_key_from_agent(
                &username
                    .map(|s| s.to_string())
                    .or_else(|| helper.username.to_owned())
                    .unwrap_or_else(|| "git".to_string()),
            )
        } else if kind.is_default() {
            Cred::default()
        } else if kind.is_user_pass_plaintext() {
            Cred::credential_helper(&config, url, username)
        } else {
            Err(git2::Error::from_str(&format!(
                "authentication '{:?}' is not available",
                kind
            )))
        }
    });

    let mut opts = FetchOptions::new();
    opts.remote_callbacks(callback);
    opts.download_tags(AutotagOption::All);
    opts
}

pub fn clone(origin: &PathBuf, path: &PathBuf) -> Result<Repository, Box<dyn Error + Send + Sync>> {
    let mut builder = RepoBuilder::new();
    builder.fetch_options(build_fetch_options());

    Ok(builder
        .clone(origin.to_string_lossy().as_ref(), path)
        .map_err(|err| GitError::Clone {
            origin: origin.to_owned(),
            path: path.to_owned(),
            err,
        })?)
}

pub fn pull(repository: &cfg::Repository) -> Result<Repository, Box<dyn Error + Send + Sync>> {
    let repo = Repository::open(&repository.path).map_err(|err| GitError::Pull {
        path: repository.path.to_owned(),
        err,
    })?;

    Ok(repo)
}
