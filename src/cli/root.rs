use crate::cli::{build, create, install, run, uninstall};
use clap::{Parser, Subcommand};
use std::env;

/// Main structure of the CPM command-line interface
#[derive(Parser)]
#[command(name = "cpm")]
#[command(author)]
#[command(about = "The C/C++ Project Manager")]
#[command(
    long_about = "The C/C++ Project Manager is a simple C/C++ project management tool,
    similar to Rust's Cargo, but designed specifically for C/C++ projects."
)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Enumeration of commands supported by CPM
// This enumeration defines all the subcommands supported by the CPM tool.
// Each variant corresponds to a specific command-line operation.
#[derive(Subcommand)]
pub enum Commands {
    /// Create a new C/C++ project
    #[command(arg_required_else_help = true)]
    Create {
        /// Name of the project to create
        name: String,
    },

    /// Install the current C/C++ project with dist structure
    #[command(arg_required_else_help = false)]
    Install {
        /// Package URL or name to install (defaults to current directory for local installation)
        #[arg(default_value = ".")]
        url: String,

        /// Install as development dependency (writes to [dev_dependencies])
        #[arg(short = 'D', long = "dev", default_value = "false")]
        dev: bool,
    },

    /// Uninstall the current C/C++ project with dist structure
    #[command(arg_required_else_help = false)]
    Uninstall {
        /// Package URL or name to uninstall (defaults to current directory for local uninstallation)
        #[arg(default_value = ".")]
        url: String,
    },

    /// Build the current C/C++ project with dist structure
    #[command(arg_required_else_help = false)]
    Build {
        /// Path to the project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Run the current C/C++ project with dist structure
    #[command(arg_required_else_help = false)]
    Run {
        /// Path to the project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },
}

/// Execute the CLI command
/// Perform the corresponding operation based on the sub-command selected by the user.
/// This method consumes `self` because the CLI state is no longer needed after the command execution.
/// # Return
/// - `Ok(())`: Command executed successfully /// - `Err(...) : An error occurred during the command execution process
/// # Error Handling
/// All errors will be wrapped in `Box<dyn std::error::Error + Send + Sync>` and returned,
/// so that they can be uniformly handled and presented with friendly error messages in the main program.
/// # Example
///  ```ignore
/// let cli = Cli::parse();
/// if let Err(e) = cli.run() {
///     eprintln! ("Error: {}", e);
///     std::process::exit(1);
/// }
/// ```
impl Cli {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Execute the corresponding command based on the user's input
        use Commands::{Build, Create, Install, Run, Uninstall};
        match self.command {
            Create { name } => {
                create::create_project(&name)?;
            }
            Install { url, dev } => {
                // Check if the url is a local directory or a remote package
                if url == "." {
                    // When the path is set to the default value,
                    // the function "install_all" (which installs all the dependencies of the current project) will be executed.
                    install::install_all();
                } else {
                    // When the path is not set to the default value,
                    // the function "install" (which installs the specified package) will be executed.
                    install::install(&url, dev)?;
                }
            }
            Uninstall { url } => {
                if url == "." {
                    uninstall::uninstall_all()?;
                } else {
                    uninstall::uninstall(&url)?;
                }
            }
            Build { path } => {
                // Get project root directory path
                let actual_path = if path == "." {
                    env::current_dir()
                        .unwrap_or_else(|_| std::path::PathBuf::from("."))
                        .to_string_lossy()
                        .into_owned()
                } else {
                    path
                };
                build::build_project(&actual_path)?;
            }
            Run { path } => {
                let actual_path = if path == "." {
                    env::current_dir()
                        .unwrap_or_else(|_| std::path::PathBuf::from("."))
                        .to_string_lossy()
                        .into_owned()
                } else {
                    path
                };
                run::run_project(&actual_path)?;
            }
        }
        Ok(())
    }
}
