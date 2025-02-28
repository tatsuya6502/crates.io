use crate::exit_status_ext::ExitStatusExt;
use secrecy::{ExposeSecret, SecretString};
use std::path::Path;
use std::process::Command;

pub fn new_lib(parent_path: &Path, name: &str) -> anyhow::Result<()> {
    Command::new("cargo")
        .args(["new", "--lib", name])
        .current_dir(parent_path)
        .env("CARGO_TERM_COLOR", "always")
        .status()?
        .exit_ok()
        .map_err(Into::into)
}

pub fn publish(project_path: &Path, token: &SecretString) -> anyhow::Result<()> {
    Command::new("cargo")
        .args(["publish", "--registry", "staging", "--allow-dirty"])
        .current_dir(project_path)
        .env("CARGO_TERM_COLOR", "always")
        .env(
            "CARGO_REGISTRIES_STAGING_INDEX",
            "https://github.com/rust-lang/staging.crates.io-index",
        )
        .env("CARGO_REGISTRIES_STAGING_TOKEN", token.expose_secret())
        .status()?
        .exit_ok()
        .map_err(Into::into)
}
