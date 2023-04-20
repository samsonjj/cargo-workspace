#![feature(provide_any)]
#![feature(error_generic_member_access)]

use std::backtrace::Backtrace;
use std::fs::read;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SecretsError>;

#[derive(Error, Debug)]
pub enum SecretsError {
    #[error("secrets file missing at {0:?}")]
    MissingFile(PathBuf),
    #[error("could not locate home directory")]
    MissingHome,
    #[error("io error")]
    Io {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },
    #[error("error parsing toml file")]
    Toml {
        #[from]
        source: toml::de::Error,
        backtrace: Backtrace,
    },
    #[error("error accessing key `{0}`")]
    TomlGet(String),
    #[error("error parsing value as string")]
    TomlParseValue,
}

/// Temporary solution for grabbing db password.
///
/// In the future we should use thrift types or something to generate
/// config / secrets in a strongly typed way.
pub fn get_database_password() -> Result<String> {
    let table = get_secrets_table()?;
    dbg!(&table);
    Ok(table
        .get("database")
        .ok_or(SecretsError::TomlGet("database".to_string()))?
        .get("password")
        .ok_or(SecretsError::TomlGet("password".to_string()))?
        .as_str()
        .ok_or(SecretsError::TomlParseValue)?
        .to_string())
}

pub fn get_secrets_table() -> Result<toml::Table> {
    let file_string = read_secrets_file()?;
    let table = file_string.parse::<toml::Table>()?;
    Ok(table)
}

fn read_secrets_file() -> Result<String> {
    let file_path = get_secrets_path()?;
    Ok(std::fs::read_to_string(file_path)?)
}

fn get_secrets_path() -> Result<std::path::PathBuf> {
    // only works on linux, that's fine
    let path = std::env::home_dir()
        .ok_or(SecretsError::MissingHome)?
        .join(".secrets")
        .join("secrets.toml");

    dbg!(&path);

    Ok(path)
}
