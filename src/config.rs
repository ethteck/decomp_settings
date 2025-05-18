use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    path::PathBuf,
};

#[cfg(feature = "python_bindings")]
use pyo3::prelude::*;

use serde::Deserialize;

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
/// Settings for a version in frogress's schema
pub struct FrogressVersionOpts {
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
/// Settings for [frogress](https://github.com/decompals/frogress)
pub struct FrogressOpts {
    /// The project slug as defined in the frogress database
    pub project: String,
    /// The frogress settings for each version of the project
    pub versions: HashMap<String, FrogressVersionOpts>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, module = "decomp_settings")
)]
pub struct AnyOpts(serde_yaml::Value);

impl AnyOpts {
    pub fn into_inner(self) -> serde_yaml::Value {
        self.0
    }
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
#[serde(untagged)]
/// Represents a tool and its settings
pub enum ToolOpts {
    Decompme(DecompmeOpts),
    Permuter(PermuterOpts),
    Frogress(FrogressOpts),
    Other(AnyOpts),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(
    feature = "python_bindings",
    pyclass(frozen, get_all, module = "decomp_settings")
)]
/// Represents a single version of a project. A project can have one or several versions. For example, one might decompile the JP 1.0, JP 1.1, and US 1.1 versions of a game. These each would be a Version.
pub struct Version {
    /// The shorthand identifier for the version, meant to be easily memorable and typable. Example: us10
    pub name: String,
    /// The human-readable name of the version. Example: US 1.0
    pub fullname: String,
    /// The sha1 hash of the target executable. This can be used by tools to ensure the correct executable is being worked with.
    pub sha1: Option<String>,
    /// A map of path names to paths that tools may care about. Common paths would be baserom, asm, build, map, expected, etc.
    pub paths: HashMap<String, PathBuf>,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.fullname))
    }
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
    /// A list of all Versions inside the project
    pub versions: Vec<Version>,
    /// Settings for various tools
    pub tools: Option<HashMap<String, ToolOpts>>,
}

#[cfg_attr(feature = "python_bindings", pymethods)]
impl Config {
    pub fn get_version_by_name(&self, version: &str) -> Option<Version> {
        self.versions.iter().find(|v| v.name == version).cloned()
    }
}
