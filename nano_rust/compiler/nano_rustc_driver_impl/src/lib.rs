use std::process::{ExitCode, Termination};

pub use nano_rustc_span::fatal_error::catch_fatal_errors;

use nano_rustc_session::{config, getopts};

pub mod args;

pub enum HandledOptions {
    /// Parsing failed, or we parsed a flag causing an early exit
    None,
    /// Successful parsing
    Normal(getopts::Matches),
    /// Parsing succeeded, but we received one or more `help` flags
    /// The compiler should proceed only until a possible `-W help` flag has been processed
    HelpOnly(getopts::Matches),
}

/// Process command line options. Emits messages as appropriate. If compilation
/// should continue, returns a getopts::Matches object parsed from args,
/// otherwise returns `None`
fn handle_options(args: &[String]) -> HandledOptions {
    // Parse with *all* options defined in the compiler
    let mut options = getopts::Options::new();
    let optgroups = config::nano_rustc_optgroups();
    for option in &optgroups {
        option.apply(&mut options);
    }
    let matches = options.parse(args).unwrap_or_else(|e| {
        panic!("{e}");
    });
    HandledOptions::Normal(matches)
}

/// This is the primary entry point for nano_rustc.
pub fn run_compiler(at_args: &[String]) {
    let at_args = at_args.get(1..).unwrap_or_default();

    let args = args::arg_expand_all(at_args);
}

/// Variant of `catch_fatal_erros` for the `interface::Result` return type
/// that also computes the exit code
pub fn catch_with_exit_code<T: Termination>(f: impl FnOnce() -> T) -> ExitCode {
    match catch_fatal_errors(f) {
        Ok(status) => status.report(),
        _ => ExitCode::FAILURE,
    }
}

pub fn main() -> ExitCode {
    let exit_code = catch_with_exit_code(|| run_compiler(&args::raw_args()));

    exit_code
}
