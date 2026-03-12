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
