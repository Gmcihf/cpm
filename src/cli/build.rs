use crate::builder::collect::{BuildConfig, Builder};
use crate::builder::gpp::GppCompiler;
use crate::core::toml::operation::OperationToml;

/// The main function for building the project - a concise imperative process
pub fn build_project(project_path: &str) -> Result<(), String> {
    // 1. Read the configuration file
    let parse = OperationToml::new(&format!("{}/cpm.toml", project_path));
    let compiler_name = parse.get("build", "compiler");
    let flags = parse.get_flags();
    let dependencies = parse.list(false);
    let system_libraries = parse.get_system_libraries();

    // 2. Get the output type configuration (default is "bin")
    let output_type = parse.get("build", "output");
    let output_type = if output_type.is_empty() {
        "bin"
    } else {
        output_type.as_str()
    };

    // 3. create a new build config instance
    let config = BuildConfig::new_with_output(
        project_path,
        &compiler_name,
        flags,
        &dependencies,
        &system_libraries,
        output_type,
    );

    // 4. Create a new builder instance based on the compiler name
    let builder = match compiler_name.as_str() {
        "g++" => Builder::new(Box::new(GppCompiler)),
        "gcc" => {
            return Err(
                "\x1b[33m⚠️  The compiler has not been implemented yet.\x1b[0m".to_string(),
            );
        }
        "clang" => {
            return Err(
                "\x1b[33m⚠️  The compiler has not been implemented yet.\x1b[0m".to_string(),
            );
        }
        "clang++" => {
            return Err(
                "\x1b[33m⚠️  The compiler has not been implemented yet.\x1b[0m".to_string(),
            );
        }
        _ => {
            return Err(format!(
                "\x1b[31m❌ Unsupported compiler: {}\x1b[0m",
                compiler_name
            ));
        }
    };

    // 5. Build the project
    builder.build(&config)?;

    Ok(())
}
