use std::fs;
use std::path::Path;
use std::process::Command;

/// Clone the code from the Git repository to the specified directory
///
/// # Arguments
/// * `url` - Git repository URL
/// * `target_dir` - Target directory paths
pub fn clone_repository(url: &str, target_dir: &str) {
    // Expand the ~ symbol in the path
    let expanded_path = shellexpand::tilde(target_dir);

    // Check if .git file already exists (already a git repository)
    let git_path = Path::new(expanded_path.as_ref()).join(".git");
    if git_path.exists() {
        println!(
            "⚠️  Directory already contains .git file, skipping clone: {}",
            expanded_path.as_ref()
        );
        return;
    }

    // Check if directory exists and is not empty
    if Path::new(expanded_path.as_ref()).exists() {
        // Directory exists but no .git file, need to delete and re-clone
        println!(
            "⚠️  Directory exists but no .git file, deleting old directory: {}",
            expanded_path.as_ref()
        );
        fs::remove_dir_all(expanded_path.as_ref()).expect("❌ Failed to delete old directory");
    }

    println!("🔄 Cloning repository: {}", url);

    // Execute git clone command, output directly to terminal
    Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(url)
        .arg(expanded_path.as_ref())
        .status()
        .expect("❌ Failed to execute git clone command");

    println!("✅ Repository clone completed: {}", expanded_path.as_ref());
}

/// Execute CMake configuration
///
/// # Arguments
/// * `path` - Build directory path
/// * `cmake ..`
pub fn cmake_build(path: &str) {
    // Expand the ~ symbol in the path
    let expanded_path = shellexpand::tilde(path);

    println!("🔨 Executing cmake: {}", expanded_path.as_ref());

    // Enter path directory and execute cmake ..
    Command::new("cmake")
        .arg("..")
        .current_dir(expanded_path.as_ref())
        .status()
        .expect("❌ Failed to execute cmake command");

    println!("✅ cmake configuration completed");
}

/// Execute Make compilation
///
/// # Arguments
/// * `path` - Build directory path
/// * `make .`
pub fn make_build(path: &str) {
    // Expand the ~ symbol in the path
    let expanded_path = shellexpand::tilde(path);

    println!("🔨 Executing make: {}", expanded_path.as_ref());

    // Enter path directory and execute make
    Command::new("make")
        .current_dir(expanded_path.as_ref())
        .status()
        .expect("❌ Failed to execute make command");

    println!("✅ make compilation completed");
}

/// Move compiled library files and header files to specified directories
///
/// # Arguments
/// * `dist_path` - Build output directory path (contains build and include subdirectories)
/// * `include_path` - Target header file directory path
/// * `lib_path` - Target library file directory path
pub fn move_code(dist_path: &str, include_path: &str, lib_path: &str) {
    // Expand the ~ symbol in the path
    let expanded_dist_path = shellexpand::tilde(dist_path);
    let expanded_include_path = shellexpand::tilde(include_path);
    let expanded_lib_path = shellexpand::tilde(lib_path);

    println!("📦 Starting to move files...");

    // Create target directories
    fs::create_dir_all(expanded_include_path.as_ref())
        .expect("❌ Failed to create include directory");
    fs::create_dir_all(expanded_lib_path.as_ref()).expect("❌ Failed to create library directory");

    // 1. Process library files
    let lib_dir = format!("{}/build", expanded_dist_path.as_ref());
    if Path::new(&lib_dir).exists() {
        println!("📚 Processing library files: {}", lib_dir);

        // Iterate through all files in build directory
        for entry in fs::read_dir(&lib_dir).expect("❌ Failed to read build directory") {
            let entry = entry.expect("❌ Failed to read directory entry");
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy();
                // Check if it's a library file (.a, .so, .dll, .lib)
                if file_name.ends_with(".a")
                    || file_name.ends_with(".so")
                    || file_name.ends_with(".dll")
                    || file_name.ends_with(".lib")
                {
                    let dest_path = Path::new(expanded_lib_path.as_ref()).join(&*file_name);
                    println!(
                        "  → Moving library file: {} -> {}",
                        path.display(),
                        dest_path.display()
                    );
                    fs::copy(&path, &dest_path).expect("❌ Failed to copy library file");
                }
            }
        }
    } else {
        eprintln!("⚠️  Build directory does not exist: {}", lib_dir);
    }

    // 2. Process header files
    let include_dir = format!("{}/include", expanded_dist_path.as_ref());
    if Path::new(&include_dir).exists() {
        println!("📄 Processing header files: {}", include_dir);

        // Recursively copy all files and folders in include directory
        copy_dir_contents(
            Path::new(&include_dir),
            Path::new(expanded_include_path.as_ref()),
        )
        .expect("❌ Failed to copy header files");
    } else {
        eprintln!("⚠️  Include directory does not exist: {}", include_dir);
    }

    println!("✅ File movement completed");
}

/// Recursively copy directory contents
///
/// # Arguments
/// * `src_dir` - Source directory path
/// * `dst_dir` - Target directory path
fn copy_dir_contents(src_dir: &Path, dst_dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst_dir.join(entry.file_name());

        if src_path.is_dir() {
            // If it's a directory, recursively copy
            fs::create_dir_all(&dst_path)?;
            copy_dir_contents(&src_path, &dst_path)?;
        } else {
            // If it's a file, copy directly
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
