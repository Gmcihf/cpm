use crate::config::cppdata::{CPM_FILE, GITIGNORE_FILE, HEADER_FILE, MAIN_FILE, README_FILE};
use crate::utils::path::operation::env_path;
use std::collections::HashMap;
use std::fs;
use std::io;

///Create project
///
/// # Arguments
/// * `project_name` - Project name
///
/// # Returns
/// * `Ok(())` - Project created successfully
/// * `Err(io::Error)` - Error message
pub fn create_project(project_name: &str) -> Result<(), io::Error> {
    let base_path = env_path().map_err(io::Error::other)?;
    let project_path = format!("{}/{}", base_path, project_name);

    // Create the project directory
    fs::create_dir_all(&project_path)?;

    // create standard directory structure：include/ and src/
    let include_dir = format!("{}/include", project_path);
    let src_dir = format!("{}/src", project_path);
    fs::create_dir_all(&include_dir)?;
    fs::create_dir_all(&src_dir)?;

    // generate the uppercase form of the project name (for header guards)
    let project_name_upper = project_name.to_uppercase().replace('-', "_");

    let mut files = HashMap::new();
    files.insert(
        "src/main.cpp".to_string(),
        MAIN_FILE.replace("{project_name}", project_name),
    );
    files.insert(
        format!("include/{}.hpp", project_name),
        HEADER_FILE
            .replace("{project_name}", project_name)
            .replace("{project_name_upper}", &project_name_upper),
    );
    files.insert(
        "cpm.toml".to_string(),
        CPM_FILE.replace("{project_name}", project_name),
    );
    files.insert(
        "README.md".to_string(),
        README_FILE.replace("{project_name}", project_name),
    );
    files.insert(".gitignore".to_string(), GITIGNORE_FILE.to_string());

    for (file_path, content) in files.iter() {
        // Create the full path for the file
        let full_path = format!("{}/{}", project_path, file_path);

        // Create parent directories if they don't exist
        if let Some(parent) = std::path::Path::new(&full_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // Write content
        fs::write(&full_path, content)?;
    }

    println!("1. cd {}", project_name);
    println!("2. cpm build and cpm run");
    Ok(())
}
