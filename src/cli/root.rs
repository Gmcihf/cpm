use clap::{Parser, Subcommand};

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
    /// print "Hello, World!" message
    Hello,
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
        match self.command {
            Commands::Hello => println!("Hello, World!"),
        }
        Ok(())
    }
}
