use std::fs;
use std::path::Path;

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
