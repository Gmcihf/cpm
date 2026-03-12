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
