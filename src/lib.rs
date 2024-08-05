pub mod config;
pub mod error;

use std::{fs::metadata, path::Path};

use config::Config;
use error::DecompSettingsError;

#[cfg_attr(feature = "python_bindings", pyfunction)]
/// Looks for a configuration file named `decomp.yaml` starting from the current directory and going to all parent directories.
pub fn scan_for_config() -> Result<Config, DecompSettingsError> {
    let path = std::env::current_dir().unwrap();
    scan_for_config_from(path.to_str().unwrap())
}

#[cfg_attr(feature = "python_bindings", pyfunction)]
/// Looks for a configuration file named `decomp.yaml` starting from the given directory and going to all parent directories.
pub fn scan_for_config_from(start: &str) -> Result<Config, DecompSettingsError> {
    match metadata(start) {
        Ok(md) => {
            if !md.is_dir() {
                return Err(DecompSettingsError::ConfigScanError(start.to_string()));
            }
        }
        Err(_) => {
            return Err(DecompSettingsError::ConfigScanError(start.to_string()));
        }
    }

    let mut path: &Path = Path::new(start);

    loop {
        let maybe_here = path.join("decomp.yaml");

        if let Ok(md) = metadata(&maybe_here) {
            if md.is_file() {
                return read_config(maybe_here.to_str().unwrap());
            }
        }

        if path.parent().is_none() {
            break;
        }
        path = path.parent().unwrap();
    }

    Err(DecompSettingsError::ConfigNotFound(start.to_string()))
}

#[cfg_attr(feature = "python_bindings", pyfunction)]
/// Reads a configuration file from the given path.
pub fn read_config(path: &str) -> Result<Config, DecompSettingsError> {
    let md = metadata(path);
    match md {
        Ok(md) => {
            if !md.is_file() {
                return Err(DecompSettingsError::ConfigReadError(path.to_string()));
            }
        }
        Err(_) => {
            return Err(DecompSettingsError::ConfigReadError(path.to_string()));
        }
    }
    let config: Config = serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    Ok(config)
}

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[cfg(feature = "python_bindings")]
#[pymodule]
fn decomp_settings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scan_for_config, m)?)?;
    m.add_function(wrap_pyfunction!(scan_for_config_from, m)?)?;
    m.add_function(wrap_pyfunction!(read_config, m)?)?;
    m.add_class::<Config>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config() {
        let config = read_config("test/decomp.yaml").unwrap();
        assert_eq!(config.platform, "n64");
    }

    #[test]
    fn test_scan_for_config() {
        let result = scan_for_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_for_config_from() {
        let config = scan_for_config_from("test/subdir").unwrap();
        assert!(config.platform == "n64");
    }

    #[test]
    fn test_get_default_version() {
        let mut config = read_config("test/decomp.yaml").unwrap();
        let version = config.get_default_version().unwrap();
        assert_eq!(version.name, "US");

        config.default_version = None;
        let result = config.get_default_version();
        assert!(result.is_err());
    }
}
