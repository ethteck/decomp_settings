#[cfg(feature = "python_bindings")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecompSettingsError {
    #[error("Could not find config file at {0}")]
    ConfigNotFound(String),
    #[error("Could not read config file at {0}")]
    ConfigReadError(String),
    #[error("Could not scan for config from {0} because it is a file")]
    ConfigScanError(String),
    #[error("Version {0} not defined in config")]
    VersionNotFound(String),
    #[error("Tried to get the default version, but no default_version is set in the config")]
    NoDefaultVersion,
}

#[cfg(feature = "python_bindings")]
impl std::convert::From<DecompSettingsError> for PyErr {
    fn from(err: DecompSettingsError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}
