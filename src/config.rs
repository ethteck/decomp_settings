use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathsOpts {
    pub baserom: String,
    pub build: String,
    pub nonmatchings: String,
    pub map: String,
    pub elf: String,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecompmeOpts {
    pub preset: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermuterOpts {
    pub decompme_compilers: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M2cOpts {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
pub struct Config {
    pub platform: String, // TODO maybe type
    pub frogress_project: Option<String>,
    pub default_version: Option<String>,
    pub versions: Vec<Version>,
}
