use crate::core::package::repository::{clone_repository, cmake_build, make_build, move_code};
use crate::core::toml::operation::OperationToml;
use crate::utils::file::operation::get_repo_name;
use crate::utils::path::operation::env_path;
use crate::utils::path::operation::{create_dir, delete_dir};

/// Main entry point for installation
///
/// # Arguments
/// * `url` - Git repository URL or package name
/// * `dev` - Whether it's a development dependency (true writes to [dev_dependencies], false writes to [dependencies])
pub fn install(url: &str, dev: bool) -> Result<(), String> {
    println!("\x1b[34m🚀 Starting build process...\x1b[0m\n");

    match build_project(url, dev) {
        Ok(_) => {
            println!("\x1b[32m🎉 Build completed!\x1b[0m");
            Ok(())
        }
        Err(e) => {
            eprintln!("\x1b[31m❌ Build failed: {}\x1b[0m", e);
            std::process::exit(1);
        }
    }
}

/// Install all dependencies from cpm.toml
pub fn install_all() {
    println!("\x1b[34m🚀 ALL Starting build process...\x1b[0m\n");
    let base_path = env_path().expect("Failed to get current directory");
    let cpm_path = format!("{}/cpm.toml", base_path);
    println!(
        "\x1b[33m📋 Reading configuration file: {}\x1b[0m\n",
        cpm_path
    );

    let parse = OperationToml::new(cpm_path.as_str());
    let packages = parse.list(false);

    println!(
        "\x1b[33m📦 Found {} dependency packages:\x1b[0m\n",
        packages.len()
    );

    for (name, url) in packages {
        println!("   \x1b[36m📌 {}: {}\x1b[0m", name, url);
        match build_project_for_all(url.as_str()) {
            Ok(_) => println!("\x1b[32m   ✅ Build completed!\x1b[0m\n"),
            Err(e) => {
                eprintln!("\x1b[31m   ❌ Build failed: {}\x1b[0m", e);
                std::process::exit(1);
            }
        }
    }

    // Clean up temporary directory after all packages are built
    println!("\x1b[33m🗑️  Cleaning temporary working directory\x1b[0m");
    let temp_dir = format!("{}/temp", base_path);
    delete_dir(temp_dir.as_str()).expect("Failed to delete temp directory");
    println!("   Cleaned: {}\n", temp_dir);

    println!("\x1b[32m🎉 All dependency packages built successfully!\x1b[0m");
}

/// Execute complete project build process (for single package installation)
///
/// # Arguments
/// * `url` - Git repository URL
/// * `dev` - Whether it's a development dependency (true writes to [dev_dependencies], false writes to [dependencies])
///
/// # Returns
/// * `Result<(), String>` - Ok on success, error message on failure
fn build_project(url: &str, dev: bool) -> Result<(), String> {
    // Basic directory configuration
    let base_dir = env_path()?;

    // Step 1: Get repository name
    println!("\x1b[33m📋 Step 1: Parsing repository information\x1b[0m");
    let repo_name = get_repo_name(url);
    println!("   Repository name: {}\n", repo_name);

    // Step 2: Create temporary working directory
    println!("\x1b[33m📁 Step 2: Creating working directory\x1b[0m");
    let temp_dir = format!("{}/temp", base_dir);
    create_dir(temp_dir.as_str())?;
    let repo_path = format!("{}/{}", temp_dir, repo_name);
    println!("   Working directory: {}\n", repo_path);

    // Step 3: Clone repository code
    println!("\x1b[33m📥 Step 3: Cloning repository code\x1b[0m");
    clone_repository(url, repo_path.as_str());
    println!();

    // Step 4: Create build directory and execute cmake
    println!("\x1b[33m🔧 Step 4: CMake configuration\x1b[0m");
    let build_dir = format!("{}/build", repo_path);
    create_dir(build_dir.as_str())?;
    cmake_build(build_dir.as_str());
    println!();

    // Step 5: Execute make compilation
    println!("\x1b[33m⚙️  Step 5: Make compilation\x1b[0m");
    make_build(build_dir.as_str());
    println!();

    // Step 6: Move build artifacts to module directory
    println!("\x1b[33m📦 Step 6: Organizing build artifacts\x1b[0m");
    let modules_base = format!("{}/modules/{}", base_dir, repo_name);
    let include_dir_target = format!("{}/include", modules_base);
    let lib_dir_target = format!("{}/lib", modules_base);

    create_dir(include_dir_target.as_str())?;
    create_dir(lib_dir_target.as_str())?;

    println!("   Header file directory: {}", include_dir_target);
    println!("   Library file directory: {}", lib_dir_target);

    move_code(
        repo_path.as_str(),
        include_dir_target.as_str(),
        lib_dir_target.as_str(),
    );
    println!();

    // Step 7: Clean up temporary directory (only delete current package's subdirectory)
    println!("\x1b[33m🗑️  Step 7: Cleaning temporary directory\x1b[0m");
    delete_dir(repo_path.as_str())?;
    println!("   Cleaned: {}\n", repo_path);

    // Step 8: Write to TOML file (decide which section to write based on dev parameter)
    println!("\x1b[33m📝 Step 8: Writing to TOML file\x1b[0m");
    let toml_path = format!("{}/{}", base_dir, "cpm.toml");
    let mut parse = OperationToml::new(toml_path.as_str());

    if dev {
        // Write to development dependencies
        parse.add_dev_dependency(&repo_name, url);
        println!("   Added to [dev_dependencies]: {} -> {}", repo_name, url);
    } else {
        // Write to regular dependencies
        parse.add_dependency(&repo_name, url);
        println!("   Added to [dependencies]: {} -> {}", repo_name, url);
    }

    parse.save();
    println!("   TOML file saved: {}\n", toml_path);

    Ok(())
}

/// Execute complete project build process (for batch installation)
///
/// # Arguments
/// * `url` - Git repository URL
///
/// # Returns
/// * `Result<(), String>` - Ok on success, error message on failure
fn build_project_for_all(url: &str) -> Result<(), String> {
    // Basic directory configuration
    let base_dir = env_path()?;

    // Step 1: Get repository name
    println!("\x1b[33m📋 Step 1: Parsing repository information\x1b[0m");
    let repo_name = get_repo_name(url);
    println!("   Repository name: {}\n", repo_name);

    // Step 2: Create temporary working directory
    println!("\x1b[33m📁 Step 2: Creating working directory\x1b[0m");
    let temp_dir = format!("{}/temp", base_dir);
    create_dir(temp_dir.as_str())?;
    let repo_path = format!("{}/{}", temp_dir, repo_name);
    println!("   Working directory: {}\n", repo_path);

    // Step 3: Clone repository code
    println!("\x1b[33m📥 Step 3: Cloning repository code\x1b[0m");
    clone_repository(url, repo_path.as_str());
    println!();

    // Step 4: Create build directory and execute cmake
    println!("\x1b[33m🔧 Step 4: CMake configuration\x1b[0m");
    let build_dir = format!("{}/build", repo_path);
    create_dir(build_dir.as_str())?;
    cmake_build(build_dir.as_str());
    println!();

    // Step 5: Execute make compilation
    println!("\x1b[33m⚙️  Step 5: Make compilation\x1b[0m");
    make_build(build_dir.as_str());
    println!();

    // Step 6: Move build artifacts to module directory
    println!("\x1b[33m📦 Step 6: Organizing build artifacts\x1b[0m");
    let modules_base = format!("{}/modules/{}", base_dir, repo_name);
    let include_dir_target = format!("{}/include", modules_base);
    let lib_dir_target = format!("{}/lib", modules_base);

    create_dir(include_dir_target.as_str())?;
    create_dir(lib_dir_target.as_str())?;

    println!("   Header file directory: {}", include_dir_target);
    println!("   Library file directory: {}", lib_dir_target);

    move_code(
        repo_path.as_str(),
        include_dir_target.as_str(),
        lib_dir_target.as_str(),
    );
    println!();

    // Step 7: Write to TOML file
    println!("\x1b[33m📝 Step 7: Writing to TOML file\x1b[0m");
    let toml_path = format!("{}/{}", base_dir, "cpm.toml");
    let mut parse = OperationToml::new(toml_path.as_str());
    parse.add_dependency(&repo_name, url);
    parse.save();
    println!("   TOML file saved: {}", toml_path);
    println!();

    Ok(())
}
