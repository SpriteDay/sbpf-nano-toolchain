use std::process::{ExitCode, Termination};

pub use nano_rustc_span::fatal_error::catch_fatal_errors;

pub mod args;

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
