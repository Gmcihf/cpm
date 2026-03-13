use crate::builder::compiler_trait::Compiler;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// G++ Compiler Implementation
pub struct GppCompiler;

impl Compiler for GppCompiler {
    fn name(&self) -> &str {
        "g++"
    }

    fn compile(&self, source: &str, output: &str, flags: &[String]) -> Result<(), String> {
        let mut cmd = Command::new("g++");

        // add source file
        cmd.arg(source);

        // add compiler flags
        for flag in flags {
            cmd.arg(flag);
        }

        // add compile flag
        cmd.arg("-c");
        cmd.arg(format!("-o{}", output));

        // inherit standard output and error
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        // execute compilation
        match cmd.status() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "Compilation failed with exit code: {:?}",
                        status.code()
                    ))
                }
            }
            Err(e) => Err(format!("Failed to execute g++ compiler: {}", e)),
        }
    }

    fn link(
        &self,
        objects: &[String],
        output: &str,
        lib_dirs: &[PathBuf],
        libraries: &[String],
        output_type: &str,
    ) -> Result<(), String> {
        match output_type {
            "static" => {
                // static library: use ar tool
                let mut cmd = Command::new("ar");
                cmd.arg("rcs"); // r=replace, c=create, s=create index
                cmd.arg(output);

                for obj in objects {
                    cmd.arg(obj);
                }

                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::piped());

                match cmd.status() {
                    Ok(status) if status.success() => {
                        println!(
                            "\x1b[32mSuccessfully built static library: {}\x1b[0m",
                            output
                        );
                        Ok(())
                    }
                    Ok(status) => Err(format!(
                        "Failed to create static library with exit code: {:?}",
                        status.code()
                    )),
                    Err(e) => Err(format!("Failed to execute ar: {}", e)),
                }
            }
            "shared" => {
                // dynamic library: use g++ -shared
                let mut cmd = Command::new("g++");
                cmd.arg("-shared");
                cmd.arg("-fPIC");

                for obj in objects {
                    cmd.arg(obj);
                }

                for lib_dir in lib_dirs {
                    cmd.arg(format!("-L{}", lib_dir.display()));
                }

                for lib in libraries {
                    cmd.arg(format!("-l{}", lib));
                }

                cmd.arg("-o");
                cmd.arg(output);

                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());

                match cmd.status() {
                    Ok(status) if status.success() => {
                        println!(
                            "\x1b[32mSuccessfully built shared library: {}\x1b[0m",
                            output
                        );
                        Ok(())
                    }
                    _ => Err("Failed to create shared library".to_string()),
                }
            }
            "bin" => {
                // executable file: use standard linking
                let mut cmd = Command::new("g++");

                for obj in objects {
                    cmd.arg(obj);
                }

                for lib_dir in lib_dirs {
                    cmd.arg(format!("-L{}", lib_dir.display()));
                }

                // add debug output, show libraries being linked
                println!("\x1b[36m🔗 linking libraries: \x1b[0m{:?}", libraries);

                for lib in libraries {
                    cmd.arg(format!("-l{}", lib));
                }

                cmd.arg("-o");
                cmd.arg(output);

                cmd.stdout(Stdio::inherit());
                cmd.stderr(Stdio::inherit());

                match cmd.status() {
                    Ok(status) if status.success() => {
                        println!("\x1b[32mSuccessfully built binary: {}\x1b[0m", output);
                        Ok(())
                    }
                    _ => Err("Linking failed".to_string()),
                }
            }
            _ => Err("Unsupported output type".to_string()),
        }
    }
}

/// # `Type` (helper function)
///
/// create dist output directories
pub fn create_dist_dir(project_path: &str) -> Result<(), String> {
    let base_path = PathBuf::from(project_path);
    let obj_dir = base_path.join("dist/obj");
    let out_dir = base_path.join("dist/out");

    for dir in [&obj_dir, &out_dir] {
        if !dir.exists() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }
    Ok(())
}
