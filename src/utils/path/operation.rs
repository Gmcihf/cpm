use std::fs;
use std::path::{Path, PathBuf};

/// Get the absolute path of the current working directory
///
/// # Returns
/// * `Ok(String)` - Current working directory absolute path
/// * `Err(String)` - Error message
pub fn env_path() -> Result<String, String> {
    std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))
        .and_then(|p| {
            p.into_os_string()
                .into_string()
                .map_err(|_| "Current directory path is not valid UTF-8".to_string())
        })
}

/// Create a directory if it does not exist
///
/// # Arguments
/// * `path` - Directory path (supports ~ symbol)
///
/// # Returns
/// * `Ok(String)` - Expanded full path
/// * `Err(String)` - Error message
pub fn create_dir(path: &str) -> Result<String, String> {
    let expanded_path = shellexpand::tilde(path);

    if Path::new(expanded_path.as_ref()).exists() {
        return Ok(expanded_path.to_string());
    }

    fs::create_dir_all(expanded_path.as_ref()).map_err(|e| {
        format!(
            "Failed to create directory {}: {}",
            expanded_path.as_ref(),
            e
        )
    })?;
    Ok(expanded_path.to_string())
}

/// Delete a directory if it exists
///
/// # Arguments
/// * `path` - Directory path (supports ~ symbol)
///
/// # Returns
/// * `Ok(())` - Delete success
/// * `Err(String)` - Error message
pub fn delete_dir(path: &str) -> Result<(), String> {
    let expanded_path = shellexpand::tilde(path);

    if Path::new(expanded_path.as_ref()).exists() {
        fs::remove_dir_all(expanded_path.as_ref()).map_err(|e| {
            format!(
                "Failed to delete directory {}: {}",
                expanded_path.as_ref(),
                e
            )
        })?;
    }
    Ok(())
}

/// Get all files in the src directory
///
/// # Arguments
/// * `path` - Project root directory path
///
/// # Returns
/// * `Ok(Vec<PathBuf>)` - File path list
/// * `Err(String)` - Error message
pub fn get_files_path(path: &str) -> Result<Vec<PathBuf>, String> {
    let src_path = Path::new(path).join("src");

    if !src_path.exists() {
        return Err(format!(
            "Source directory does not exist: {}",
            src_path.display()
        ));
    }

    if !src_path.is_dir() {
        return Err(format!("Path is not a directory: {}", src_path.display()));
    }

    let mut files = Vec::new();

    for entry in fs::read_dir(&src_path)
        .map_err(|e| format!("Failed to read directory {}: {}", src_path.display(), e))?
    {
        let entry =
            entry.map_err(|e| format!("Failed to read entry in {}: {}", src_path.display(), e))?;

        let file_path = entry.path();
        if file_path.is_file() {
            files.push(file_path);
        }
    }

    Ok(files)
}

/// Extract the base path from a full path (removing /src and everything after it)
///
/// # Arguments
/// * `path` - Full path string
///
/// # Returns
/// * `String` - Base path (excluding /src and everything after it)
///
/// # Example
/// ```
/// use cpm::utils::path::operation::base_path;
/// let base = base_path("/project/src/main.cpp");
/// assert_eq!(base, "/project");
/// ```
pub fn base_path(path: &str) -> String {
    path.rfind("/src")
        .map(|index| path[..index].to_string())
        .unwrap_or_default()
}
