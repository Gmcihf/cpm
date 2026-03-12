use crate::core::toml::operation::OperationToml;
use crate::utils::file::operation::get_repo_name;
use crate::utils::path::operation::{delete_dir, env_path};
use std::path::Path;

/// Uninstall the specified dependency package
pub fn uninstall(project_path: &str) -> Result<(), String> {
    // If the path doesn't exist, try to get repository name from URL
    let package_name = if !Path::new(project_path).exists() {
        get_repo_name(project_path)
    } else {
        // If it's a local path, extract directory name
        Path::new(project_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string()
    };

    println!(
        "\x1b[36m🔍 Uninstalling dependency package: {}\x1b[0m",
        package_name
    );

    // Get project root directory path
    let base_path = env_path().unwrap();
    let modules_path = format!("{}/modules/{}", base_path, package_name);

    // Check if module exists
    if !Path::new(&modules_path).exists() {
        return Err(format!(
            "\x1b[31m❌ Dependency package {} is not installed\x1b[0m",
            package_name
        ));
    }

    // Delete module directory
    delete_dir(&modules_path)?;
    println!(
        "\x1b[32m✅ Module directory deleted: {}\x1b[0m",
        modules_path
    );

    // Remove dependency from cpm.toml
    let cpm_toml_path = format!("{}/cpm.toml", base_path);
    if Path::new(&cpm_toml_path).exists() {
        let mut parse = OperationToml::new(&cpm_toml_path);

        // Try to remove regular dependency
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            parse.remove_dependency(&package_name);
        })) {
            Ok(_) => {
                parse.save();
                println!("\x1b[32m✅ Dependency configuration removed from cpm.toml\x1b[0m");
            }
            Err(_) => {
                // If not a regular dependency, try to remove development dependency
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    parse.remove_dev_dependency(&package_name);
                })) {
                    Ok(_) => {
                        parse.save();
                        println!(
                            "\x1b[32m✅ Development dependency configuration removed from cpm.toml\x1b[0m"
                        );
                    }
                    Err(_) => {
                        println!(
                            "\x1b[33m⚠️  Dependency package {} configuration not found in cpm.toml\x1b[0m",
                            package_name
                        );
                    }
                }
            }
        }
    } else {
        println!("\x1b[33m⚠️  cpm.toml file does not exist, skipping configuration update\x1b[0m");
    }

    println!(
        "\x1b[32m✨ Successfully uninstalled dependency package: {}\x1b[0m",
        package_name
    );
    Ok(())
}

/// Uninstall all dependency packages
pub fn uninstall_all() -> Result<(), String> {
    let base_path = env_path().unwrap();
    let modules_path = format!("{}/modules", base_path);

    println!("\x1b[36m🔍 Uninstalling all dependency packages...\x1b[0m");

    // Check if modules folder exists
    if !Path::new(&modules_path).exists() {
        println!("\x1b[33m⚠️  No installed dependency packages found\x1b[0m");
        return Ok(());
    }

    // Read cpm.toml to get dependency list
    let cpm_toml_path = format!("{}/cpm.toml", base_path);
    let dependencies = if Path::new(&cpm_toml_path).exists() {
        let parse = OperationToml::new(&cpm_toml_path);
        parse.list(false)
    } else {
        Vec::new()
    };

    let dev_dependencies = if Path::new(&cpm_toml_path).exists() {
        let parse = OperationToml::new(&cpm_toml_path);
        parse.list(true)
    } else {
        Vec::new()
    };

    // Delete all module directories
    delete_dir(&modules_path)?;
    println!("\x1b[32m✅ All module directories deleted\x1b[0m");

    // Clear dependency configuration from cpm.toml
    if Path::new(&cpm_toml_path).exists() {
        let mut parse = OperationToml::new(&cpm_toml_path);

        // Remove all regular dependencies
        for (name, _) in &dependencies {
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                parse.remove_dependency(name);
            }))
            .is_ok()
            {}
        }

        // Remove all development dependencies
        for (name, _) in &dev_dependencies {
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                parse.remove_dev_dependency(name);
            }))
            .is_ok()
            {}
        }

        parse.save();
        println!("\x1b[32m✅ Dependency configuration cleared from cpm.toml\x1b[0m");
    }

    println!("\x1b[32m✨ Successfully uninstalled all dependency packages\x1b[0m");
    Ok(())
}
