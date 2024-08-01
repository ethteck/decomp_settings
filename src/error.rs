use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecompSettingsError {
    #[error("Could not find config file at {0}")]
    ConfigNotFound(String),
    #[error("Could not read config file at {0}")]
    ConfigReadError(String),
    #[error("Could not scan for config from {0} because it is a file")]
    ConfigScanError(String),
}
