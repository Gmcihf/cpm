use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TOMLRule {
    pub project: Project,
    pub build: Build,
    pub dependencies: Dependencies,
    pub dev_dependencies: Dependencies,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Build {
    pub output: String,
    pub compiler: String,
    pub flags: Vec<String>,
    #[serde(default)]
    pub system_libraries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dependencies {
    #[serde(flatten)]
    pub dependencies: HashMap<String, String>,
}
