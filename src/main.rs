use clap::Parser;
use cpm::cli::root::Cli;

/// Entry point of the CPM main program
/// Responsible for parsing the command-line parameters and executing the corresponding commands.
/// If the command execution fails, it will print the error message and exit the program with exit code 1.
/// # Execution Flow
///
/// 1. Parse the command-line input parameters
/// 2. Distribute to the corresponding sub-command processing functions
/// 3. Capture and handle possible errors
/// 4. Set the appropriate exit code based on the execution result
fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
