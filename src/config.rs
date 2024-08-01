use std::collections::HashMap;

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;
use serde::Deserialize;

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
    pub platform: String, // TODO maybe type
    pub frogress_project: Option<String>,
    pub default_version: Option<String>,
    pub versions: Vec<Version>,
}
