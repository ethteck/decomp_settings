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
pub struct PathsOpts {
    pub baserom: Option<String>,
    pub build: Option<String>,
    pub asm: Option<String>,
    pub nonmatchings: Option<String>,
    pub map: Option<String>,
    pub elf: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
pub struct DecompmeOpts {
    pub preset: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
pub struct PermuterOpts {
    pub decompme_compilers: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
pub struct M2cOpts {}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
pub struct Version {
    pub name: String,
    pub shortname: String,
    pub frogress_version: String,
    pub paths: PathsOpts,
    pub decompme: Option<DecompmeOpts>,
    pub permuter: Option<PermuterOpts>,
    pub m2c: Option<M2cOpts>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
pub struct Config {
    pub name: String,
    pub github: Option<String>,
    pub website: Option<String>,
    pub discord: Option<String>,
    pub platform: String,             // TODO maybe type
    pub build_system: Option<String>, // TODO maybe type (make/ninja)
    pub frogress_project: Option<String>,
    pub default_version: Option<String>,
    pub versions: Vec<Version>,
}

impl Config {
    #[cfg_attr(feature = "python_bindings", pyfunction)]
    pub fn get_default_version(&self) -> Result<&Version, DecompSettingsError> {
        if let Some(default_version) = self.default_version.clone() {
            if let Some(version) = self.get_version_by_name(&default_version) {
                return Ok(version);
            }
            return Err(DecompSettingsError::VersionNotFound(default_version));
        }
        return Err(DecompSettingsError::NoDefaultVersion);
    }

    #[cfg_attr(feature = "python_bindings", pyfunction)]
    pub fn get_version_by_name(&self, version: &str) -> Option<&Version> {
        self.versions.iter().find(|v| v.name == version)
    }
}
