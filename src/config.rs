use std::path::PathBuf;

use crate::error::{Error, Result};

#[derive(Debug)]
pub enum ConfigError {
    CouldNotReadConfig(String),
    NoHomeDirectory(String),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for ConfigError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for ConfigError {}
// endregion: --- Error Boilerplate

#[derive(Debug)]
pub struct Config {
    pub session_key: String,
}

pub fn get_config() -> Result<Config> {
    let file_name = get_config_file_name()?;
    let session_key = std::fs::read_to_string(&file_name);
    let session_key = match session_key {
        Ok(session_key) => session_key,
        Err(session_key) => {
            let err = format!(
                "failed to read the session key from `{}` file err({})",
                &file_name.to_str().unwrap(),
                session_key
            );
            return Err(Error::Config(ConfigError::CouldNotReadConfig(err)));
        }
    };

    let session_key = session_key.trim_end().to_owned();

    Ok(Config { session_key })
}

pub fn set_config(config: Config) -> Result<()> {
    let file_name = get_config_file_name()?;

    std::fs::write(file_name, config.session_key)?;

    Ok(())
}

fn get_config_file_name() -> Result<PathBuf> {
    let home = dirs::home_dir();
    let mut home = match home {
        Some(home) => home,
        None => {
            return Err(Error::Config(ConfigError::NoHomeDirectory(
                "cannot find the home directory".to_string(),
            )))
        }
    };
    home.push(".humble-cli-key");
    Ok(home)
}
