use std::path::PathBuf;

/// Compiler trait - An interface that all specific compilers must implement
pub trait Compiler {
    /// Get the compiler name
    fn name(&self) -> &str;

    /// Compile the source code
    fn compile(&self, source: &str, output: &str, flags: &[String]) -> Result<(), String>;

    /// Link the object files to generate an executable or library
    fn link(
        &self,
        objects: &[String],
        output: &str,
        lib_dirs: &[PathBuf],
        libraries: &[String],
        output_type: &str,
    ) -> Result<(), String>;
}
