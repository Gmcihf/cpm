use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn run_project(project_path: &str) -> Result<(), String> {
    let output_dir = PathBuf::from(project_path).join("dist/out");

    // Guard clause: check if output directory exists and is readable
    let entries = std::fs::read_dir(&output_dir).map_err(|_| {
        format!(
            "Cannot read directory {}. Did you run 'cpm build'?",
            output_dir.display()
        )
    })?;

    // Find executable file using guard clauses
    let exe_path = find_executable(entries)?;

    // Execute the program with inherited stdout/stderr
    Command::new(exe_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to run program: {}", e))?;

    Ok(())
}

#[cfg(windows)]
fn find_executable(
    entries: impl Iterator<Item = std::io::Result<std::fs::DirEntry>>,
) -> Result<PathBuf, String> {
    entries
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "exe"))
        .ok_or_else(|| "No executable found in dist/out. Did you run 'cpm build'?".to_string())
}

#[cfg(not(windows))]
fn find_executable(
    entries: impl Iterator<Item = std::io::Result<std::fs::DirEntry>>,
) -> Result<PathBuf, String> {
    entries
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.is_file())
        .ok_or_else(|| "No executable found in dist/out. Did you run 'cpm build'?".to_string())
}
