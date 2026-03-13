use crate::builder::compiler_trait::Compiler;
use std::path::PathBuf;

/// Build configuration - Encapsulates all the information required for compilation and linking
pub struct BuildConfig<'a> {
    pub project_path: &'a str,
    pub compiler_name: &'a str,
    pub flags: Vec<String>,
    pub dependencies: &'a [(String, String)],
    pub system_libraries: &'a [String],
    pub output_type: &'a str,
}

impl<'a> BuildConfig<'a> {
    pub fn new(
        project_path: &'a str,
        compiler_name: &'a str,
        flags: Vec<String>,
        dependencies: &'a [(String, String)],
    ) -> Self {
        Self {
            project_path,
            compiler_name,
            flags,
            dependencies,
            system_libraries: &[],
            output_type: "bin", // Default to executable
        }
    }

    /// Create a new BuildConfig with output type specified
    pub fn new_with_output(
        project_path: &'a str,
        compiler_name: &'a str,
        flags: Vec<String>,
        dependencies: &'a [(String, String)],
        system_libraries: &'a [String],
        output_type: &'a str,
    ) -> Self {
        Self {
            project_path,
            compiler_name,
            flags,
            dependencies,
            system_libraries,
            output_type,
        }
    }
}

/// Build executor - Uses the specified compiler to perform the complete build process
pub struct Builder {
    compiler: Box<dyn Compiler>,
}

impl Builder {
    /// Create a new build executor instance
    pub fn new(compiler: Box<dyn Compiler>) -> Self {
        Self { compiler }
    }

    /// Execute the complete build process using the specified configuration
    pub fn build(&self, config: &BuildConfig) -> Result<(), String> {
        use crate::builder::gpp::create_dist_dir;

        // 1. Create output directory
        create_dist_dir(config.project_path)?;

        // 2. Add include paths to flags
        let flags = self.add_include_paths(config.flags.clone(), config.project_path);

        // 3. Compile all sources
        self.compile_all_sources(config.project_path, &flags)?;

        // 4. Collect library information (including project dependencies and system libraries)
        let (lib_dirs, libraries) =
            self.collect_library_info(config.project_path, config.dependencies);

        // 5. Merge system libraries with project dependencies
        let mut all_libraries = libraries;
        for sys_lib in config.system_libraries {
            all_libraries.push(sys_lib.clone());
        }
        println!(
            "\x1b[36m🔗 Merged library list after adding system libraries: {:?}\x1b[0m",
            all_libraries
        );

        // 6. Collect object files
        let obj_files = self.collect_object_files(config.project_path)?;

        // Guard clause: return early if no object files
        if obj_files.is_empty() {
            return Err("\x1b[31m❌ No object files found for linking\x1b[0m".to_string());
        }

        // 7. Filter object files for library builds
        let filtered_obj_files = if config.output_type == "static" || config.output_type == "shared"
        {
            self.filter_main_objects(&obj_files)?
        } else {
            obj_files
        };

        // Guard clause: return early if all files were filtered out
        if filtered_obj_files.is_empty() {
            return Err(
                "\x1b[31m❌ There are no target files to link (all files containing the main function have been filtered out)\x1b[0m"
                    .to_string(),
            );
        }

        // 8. Execute linking
        let output_file = self.get_output_file(config.project_path, config.output_type);
        self.compiler.link(
            &filtered_obj_files,
            &output_file,
            &lib_dirs,
            &all_libraries,
            config.output_type,
        )?;

        Ok(())
    }

    /// Add include paths to compiler flags
    fn add_include_paths(&self, mut flags: Vec<String>, project_path: &str) -> Vec<String> {
        use std::path::Path;

        // Step 1: Add the project include/ Directory
        let project_include_dir = Path::new(project_path).join("include");
        if project_include_dir.exists() {
            flags.push(format!("-I{}", project_include_dir.display()));
            println!(
                "\x1b[36m📚 add project include directory: {}\x1b[0m",
                project_include_dir.display()
            );
        }

        // Step 2: Add the modules/*/include/ Directory
        let modules_dir = Path::new(project_path).join("modules");

        // Guard clause: skip if modules directory doesn't exist
        if !modules_dir.exists() {
            return flags;
        }

        let entries = match std::fs::read_dir(&modules_dir) {
            Ok(entries) => entries,
            Err(_) => return flags,
        };

        for entry in entries.flatten() {
            let include_dir = entry.path().join("include");
            if include_dir.exists() {
                flags.push(format!("-I{}", include_dir.display()));
                println!(
                    "\x1b[36m📦 add dependency include directory: {}\x1b[0m",
                    include_dir.display()
                );
            }
        }

        flags
    }

    /// Compile all source files
    fn compile_all_sources(&self, project_path: &str, flags: &[String]) -> Result<(), String> {
        use crate::utils::path::operation::get_files_path;

        let files = get_files_path(project_path)?;
        for file in files {
            let base_path = crate::utils::path::operation::base_path(&file.to_string_lossy());
            let file_stem = crate::utils::file::operation::get_file_name(&file.to_string_lossy());
            let output = format!("{}/dist/obj/{}.o", base_path, file_stem);

            self.compiler
                .compile(&file.to_string_lossy(), &output, flags)?;
            println!("Compiled: {} -> {}", file.to_string_lossy(), output);
        }
        Ok(())
    }

    /// Collect library information (including project dependencies and system libraries)
    fn collect_library_info(
        &self,
        project_path: &str,
        dependencies: &[(String, String)],
    ) -> (Vec<PathBuf>, Vec<String>) {
        use std::path::Path;

        let modules_dir = Path::new(project_path).join("modules");
        let mut lib_dirs = Vec::<PathBuf>::new();
        let mut libraries = Vec::<String>::new();

        // Guard clause: return early if modules directory doesn't exist
        if !modules_dir.exists() {
            return (lib_dirs, libraries);
        }

        for (name, _url) in dependencies {
            let lib_dir = modules_dir.join(name).join("lib");

            // Guard clause: skip if lib directory doesn't exist
            if !lib_dir.exists() {
                continue;
            }

            lib_dirs.push(lib_dir.clone());

            let entries = match std::fs::read_dir(&lib_dir) {
                Ok(entries) => entries,
                Err(_) => continue,
            };

            for entry in entries.flatten() {
                let path = entry.path();

                // Guard clause: skip if not a file or doesn't have valid extension
                let Some(ext) = path.extension() else {
                    continue;
                };

                if ext != "a" && ext != "lib" {
                    continue;
                }

                // Guard clause: skip if can't get file stem
                let Some(stem) = path.file_stem() else {
                    continue;
                };

                let lib_name = stem.to_string_lossy().to_string();
                let clean_name = lib_name.strip_prefix("lib").unwrap_or(&lib_name);
                libraries.push(clean_name.to_string());
            }
        }

        (lib_dirs, libraries)
    }

    /// Get all object files in the dist/obj/ Directory
    fn collect_object_files(&self, project_path: &str) -> Result<Vec<String>, String> {
        use std::path::Path;

        let obj_dir = Path::new(project_path).join("dist/obj");
        let mut obj_files = Vec::new();

        // Guard clause: return empty if directory doesn't exist or can't be read
        let entries = match std::fs::read_dir(&obj_dir) {
            Ok(entries) => entries,
            Err(_) => return Ok(obj_files),
        };

        for entry in entries.flatten() {
            let path = entry.path();

            // Guard clause: skip directories
            if path.is_dir() {
                continue;
            }

            // Guard clause: skip files without .o extension
            let Some(ext) = path.extension() else {
                continue;
            };

            if ext != "o" {
                continue;
            }

            obj_files.push(path.to_string_lossy().to_string());
        }

        Ok(obj_files)
    }

    /// Generate the output file name based on the output_type.
    fn get_output_file(&self, project_path: &str, output_type: &str) -> String {
        // Get the project name as the base for the output file name
        let config_path = format!("{}/cpm.toml", project_path);
        let parse = crate::core::toml::operation::OperationToml::new(&config_path);
        let project_name = parse.get("project", "name");

        let exe_name = match (output_type, cfg!(windows)) {
            ("bin", true) => format!("{}.exe", project_name),
            ("bin", false) => project_name.clone(),
            ("static", _) => format!("lib{}.a", project_name),
            ("shared", true) => format!("{}.dll", project_name),
            ("shared", false) if cfg!(target_os = "macos") => format!("lib{}.dylib", project_name),
            ("shared", false) => format!("lib{}.so", project_name),
            _ => project_name.clone(),
        };

        format!("{}/dist/out/{}", project_path, exe_name)
    }

    /// Filter out object files that contain the main function (used for library builds)
    fn filter_main_objects(&self, obj_files: &[String]) -> Result<Vec<String>, String> {
        use std::process::{Command, Stdio};

        let mut filtered = Vec::new();

        for obj_file in obj_files {
            // Use nm tool to check if the object file contains the main symbol
            let output = Command::new("nm")
                .arg(obj_file)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            // Guard clause: keep file if nm command fails
            let Ok(result) = output else {
                filtered.push(obj_file.clone());
                continue;
            };

            let stdout = String::from_utf8_lossy(&result.stdout);

            // Check if the object file contains the main function symbol
            // nm output format: address type symbol_name
            // T/t represents symbols in the text section (code segment)
            let has_main = stdout
                .lines()
                .any(|line| line.split_whitespace().last() == Some("main"));

            // Guard clause: skip files containing main
            if has_main {
                println!(
                    "\x1b[33m⚠️  Filter out object file containing main function: {}\x1b[0m",
                    obj_file
                );
                continue;
            }

            filtered.push(obj_file.clone());
        }

        Ok(filtered)
    }
}
