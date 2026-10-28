/// Returns all nano rustc command line options, including metadata for
/// each option, such as whether the option is stable.
///
/// # Option style guidelines
///
/// - `<param>`: Indicates a required parameter
/// - `[param]`: Indicates an optional parameter
/// - `|`: Indicates a mutually exclusive option
/// - `*`: a list element with description
pub fn nano_rustc_optgroups() -> Vec<NanoRustcOptGroup> {}
