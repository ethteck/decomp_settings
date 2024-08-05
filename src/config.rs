use std::collections::HashMap;

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;
use serde::Deserialize;

use crate::error::DecompSettingsError;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
/// Settings for the [decomp.me](https://decomp.me) platform
pub struct DecompmeOpts {
    pub preset: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
/// Settings for [decomp-permuter](https://github.com/simonlindholm/decomp-permuter)
pub struct PermuterOpts {
    pub decompme_compilers: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
/// Settings for [m2c](https://github.com/matt-kempster/m2c)
pub struct M2cOpts {}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
/// Represents a single version of a project. A project can have one or several versions. For example, one might decompile the JP 1.0, JP 1.1, and US 1.1 versions of a game. These each would be a Version.
pub struct Version {
    /// The human-readable name of the version. Example: US 1.0
    pub name: String,
    /// The a shorthand name for the version, meant to be easily memorable and typable. Example: us10
    pub shortname: String,
    /// The sha1 hash of the target executable. This can be used by tools to ensure the correct executable is being worked with.
    pub sha1: Option<String>,
    /// The version slug as defined in the [frogress](https://github.com/decompals/frogress) database
    pub frogress_version: String,
    /// A map of path names to paths that tools may care about. Common paths would be baserom, asm, build, map, expected, etc.
    pub paths: HashMap<String, String>,
    /// Settings for the [decomp.me](https://decomp.me) platform
    pub decompme: Option<DecompmeOpts>,
    /// Settings for [decomp-permuter](https://github.com/simonlindholm/decomp-permuter)
    pub permuter: Option<PermuterOpts>,
    /// Settings for [m2c](https://github.com/matt-kempster/m2c)
    pub m2c: Option<M2cOpts>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
/// The top-level configuration struct. This represents the entirety of the decomp.yaml format.
pub struct Config {
    /// Human-readable name of the project. Example: Paper Mario
    pub name: String,
    /// The repository URL of the project
    pub github: Option<String>,
    /// The website for the project
    pub website: Option<String>,
    /// An invite link to the project's Discord server
    pub discord: Option<String>,
    /// The platform the project is for. Example: n64
    pub platform: String, // TODO maybe type
    /// The build system used by the project. Example: [ninja, make]
    pub build_system: Option<String>, // TODO maybe type (make/ninja)
    /// The project slug as defined in the [frogress](https://github.com/decompals/frogress) database
    pub frogress_project: Option<String>,
    /// The default Version of the project that tools should be considering
    pub default_version: Option<String>,
    /// A list of all Versions inside the project
    pub versions: Vec<Version>,
}

#[cfg_attr(feature = "python_bindings", pymethods)]
impl Config {
    pub fn get_default_version(&self) -> Result<Version, DecompSettingsError> {
        if let Some(default_version) = self.default_version.clone() {
            if let Some(version) = self.get_version_by_name(&default_version) {
                return Ok(version);
            }
            return Err(DecompSettingsError::VersionNotFound(default_version));
        }
        Err(DecompSettingsError::NoDefaultVersion)
    }

    pub fn get_version_by_name(&self, version: &str) -> Option<Version> {
        self.versions.iter().find(|v| v.name == version).cloned()
    }
}
