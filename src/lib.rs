pub mod config;
pub mod error;

use std::{fs::metadata, path::PathBuf};

use config::Config;
use error::DecompSettingsError;

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python_bindings", pyfunction)]
/// Looks for a configuration file named `decomp.yaml` starting from the current directory and going to all parent directories.
pub fn scan_for_config() -> Result<Config, DecompSettingsError> {
    let path = std::env::current_dir().unwrap();
    scan_for_config_from(path)
}

#[cfg_attr(feature = "python_bindings", pyfunction)]
/// Looks for a configuration file named `decomp.yaml` starting from the given directory and going to all parent directories.
pub fn scan_for_config_from(start: PathBuf) -> Result<Config, DecompSettingsError> {
    match metadata(start.clone()) {
        Ok(md) => {
            if !md.is_dir() {
                return Err(DecompSettingsError::ConfigScanError(
                    start.display().to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(DecompSettingsError::ConfigScanError(
                start.display().to_string(),
            ));
        }
    }

    let mut path = start.clone();

    loop {
        let maybe_here = path.join("decomp.yaml");

        if let Ok(md) = metadata(&maybe_here) {
            if md.is_file() {
                return read_config(maybe_here);
            }
        }

        if path.parent().is_none() {
            break;
        }
        path = path.parent().unwrap().to_path_buf();
    }

    Err(DecompSettingsError::ConfigNotFound(
        start.display().to_string(),
    ))
}

#[cfg_attr(feature = "python_bindings", pyfunction)]
/// Reads a configuration file from the given path.
pub fn read_config(path: PathBuf) -> Result<Config, DecompSettingsError> {
    match metadata(path.clone()) {
        Ok(md) => {
            if !md.is_file() {
                return Err(DecompSettingsError::ConfigReadError(
                    path.display().to_string(),
                ));
            }
        }
        Err(_) => {
            return Err(DecompSettingsError::ConfigReadError(
                path.display().to_string(),
            ));
        }
    }
    let config: Config = serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    Ok(config)
}

#[cfg(feature = "python_bindings")]
#[pymodule]
fn decomp_settings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use config::{AnyOpts, ToolOpts};

    m.add_function(wrap_pyfunction!(scan_for_config, m)?)?;
    m.add_function(wrap_pyfunction!(scan_for_config_from, m)?)?;
    m.add_function(wrap_pyfunction!(read_config, m)?)?;
    m.add_class::<Config>()?;
    m.add_class::<ToolOpts>()?;
    m.add_class::<AnyOpts>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use config::ToolOpts;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[test]
    fn test_read_config() {
        let config = read_config(PathBuf::from("test/decomp.yaml")).unwrap();
        assert_eq!(config.platform, "n64");
    }

    #[test]
    fn test_scan_for_config() {
        let result = scan_for_config();
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_for_config_from() {
        let config = scan_for_config_from(PathBuf::from("test/subdir")).unwrap();
        assert!(config.platform == "n64");
    }

    #[test]
    fn test_read_config_arbitrary_tool() {
        // Example structs that would be defined in the tool's code
        #[derive(Clone, Debug, Deserialize)]
        struct Other {
            stuff: usize,
        }

        #[derive(Clone, Debug, Deserialize)]
        struct ArbitraryTool {
            meowp: usize,
            others: Vec<HashMap<String, Other>>,
        }

        let config = read_config(PathBuf::from("test/arbitrary_tool.yaml")).unwrap();
        let tools = config.tools.unwrap();
        let arbitrary_tool_enum = tools.get("arbitrary_tool").unwrap();

        let ToolOpts::Other(tool_value) = arbitrary_tool_enum else {
            panic!("Expected ToolOpts::Other, got {:?}", arbitrary_tool_enum);
        };

        let arbitrary_tool: ArbitraryTool =
            serde_yaml::from_value(tool_value.clone().into_inner()).unwrap();

        assert_eq!(arbitrary_tool.meowp, 125);
        assert_eq!(arbitrary_tool.others[0].get("thing").unwrap().stuff, 1);
        assert_eq!(arbitrary_tool.others[1].get("thing2").unwrap().stuff, 2);
    }
}
