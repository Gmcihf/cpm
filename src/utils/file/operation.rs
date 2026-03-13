/// Get the repository name from the Git URL
///
/// # Arguments
/// * `url` - Git repository URL
///
/// # Returns
/// * `String` - Repository name (without .git suffix)
///
/// # Example
/// ```
/// use cpm::utils::file::operation::get_repo_name;
/// let name = get_repo_name("https://github.com/user/repo.git");
/// assert_eq!(name, "repo");
/// ```
pub fn get_repo_name(url: &str) -> String {
    url.trim_end_matches(".git")
        .split('/')
        .next_back()
        .unwrap_or("unknown")
        .to_string()
}

/// Extract the file name from a path (removing extension and replacing path separators with underscores)
///
/// # Arguments
/// * `file` - File path string
///
/// # Returns
/// * `String` - Processed file name (without extension, path separators replaced with underscores)
///
/// # Example
/// ```
/// use cpm::utils::file::operation::get_file_name;
/// let name = get_file_name("/project/src/main.cpp");
/// assert_eq!(name, "_src_main");
/// ```
pub fn get_file_name(file: &str) -> String {
    // Find the first occurrence of "/src" in the file path
    if let Some(src_index) = file.find("/src") {
        // Include "src" itself
        let from_src = &file[src_index..];

        // Replace "/" with "_" in the substring
        let with_underscores = from_src.replace('/', "_");

        // Remove the file extension
        if let Some(dot_pos) = with_underscores.rfind('.') {
            with_underscores[..dot_pos].to_string()
        } else {
            with_underscores
        }
    } else {
        String::new()
    }
}
