use crate::core::toml::rules::TOMLRule;
use shellexpand;
use std::fs;
use toml;

pub struct OperationToml {
    rules: TOMLRule,
    path: String,
    needs_save: bool,
}

impl OperationToml {
    /// Create a new instance of OperationToml.
    /// If the file does not exist, create a default TOML structure.
    pub fn new(path: &str) -> Self {
        // Expand the ~ symbol in the path
        let expanded_path = shellexpand::tilde(path);

        let rules = if std::path::Path::new(expanded_path.as_ref()).exists() {
            // File exists, read and parse
            let content = fs::read_to_string(expanded_path.as_ref()).expect("Error reading file");
            toml::from_str(&content).expect("Error parsing TOML")
        } else {
            //The file does not exist. Creating the default TOML structure.
            TOMLRule::default()
        };

        Self {
            rules,
            path: expanded_path.to_string(),
            needs_save: false,
        }
    }

    pub fn get_rules(&self) -> &TOMLRule {
        &self.rules
    }

    /// Get a value from the TOML file. If the key does not exist, panic.
    pub fn get(&self, key: &str, value: &str) -> String {
        let result = format!("{}.{}", key, value);
        match result.as_str() {
            "project.name" => self.rules.project.name.to_string(),
            "project.version" => self.rules.project.version.to_string(),
            "project.description" => self.rules.project.description.to_string(),
            "project.authors" => self.rules.project.authors.join(", "),
            "project.license" => self.rules.project.license.to_string(),
            "build.output" => self.rules.build.output.to_string(),
            "build.compiler" => self.rules.build.compiler.to_string(),
            "build.flags" => self.rules.build.flags.join(","),
            _ => panic!("Invalid key"),
        }
    }
    /// Get the compiler flags from the TOML file.
    /// If the TOML file does not exist, return an empty vector.
    pub fn get_flags(&self) -> Vec<String> {
        self.rules.build.flags.clone()
    }

    /// Get the system libraries from the TOML file.
    /// If the TOML file does not exist, return an empty vector.
    pub fn get_system_libraries(&self) -> Vec<String> {
        self.rules.build.system_libraries.clone()
    }
    /// Save the current state to the TOML file
    pub fn save(&mut self) {
        if self.needs_save {
            let file = toml::to_string_pretty(&self.rules).expect("Error serializing TOML");
            fs::write(&self.path, file).expect("Error writing file");
            self.needs_save = false;
        }
    }

    /// Add a dependency to the cpm.toml files
    pub fn add_dependency(&mut self, name: &str, version: &str) {
        self.rules
            .dependencies
            .dependencies
            .insert(name.to_string(), version.to_string());
        self.needs_save = true;
    }
    /// Add a dev dependency to the cpm.toml files
    pub fn add_dev_dependency(&mut self, name: &str, version: &str) {
        self.rules
            .dev_dependencies
            .dependencies
            .insert(name.to_string(), version.to_string());
        self.needs_save = true;
    }

    /// Remove a dependency (supports removing by key or value).
    pub fn remove_dependency(&mut self, identifier: &str) {
        // 首先尝试通过 key 删除
        if self
            .rules
            .dependencies
            .dependencies
            .remove(identifier)
            .is_some()
        {
            self.needs_save = true;
            return;
        }

        // If the key does not match, try to delete it based on the value.
        let mut removed = false;
        self.rules.dependencies.dependencies.retain(|_key, value| {
            if value == identifier {
                removed = true;
                false // Remove this item
            } else {
                true // Keep this item
            }
        });

        if !removed {
            panic!(
                "Dependency '{}' not found (neither a valid dependency name nor a valid dependency URL)",
                identifier
            );
        }

        self.needs_save = true;
    }

    /// Remove a development dependency (supports removing by key or value).
    pub fn remove_dev_dependency(&mut self, identifier: &str) {
        //First, try to delete it using the key.
        if self
            .rules
            .dev_dependencies
            .dependencies
            .remove(identifier)
            .is_some()
        {
            self.needs_save = true;
            return;
        }

        // IF NOT FOUND, try to delete it using the value.
        let mut removed = false;
        self.rules
            .dev_dependencies
            .dependencies
            .retain(|_key, value| {
                if value == identifier {
                    removed = true;
                    false // Remove this item
                } else {
                    true // Keep this item
                }
            });

        if !removed {
            panic!(
                "Development dependency '{}' not found (neither a valid dependency name nor a valid dependency URL)",
                identifier
            );
        }

        self.needs_save = true;
    }
    /// List all dependencies (both normal and development).s
    pub fn list(&self, is_dev: bool) -> Vec<(String, String)> {
        if is_dev {
            self.rules
                .dev_dependencies
                .dependencies
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        } else {
            self.rules
                .dependencies
                .dependencies
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        }
    }
}
