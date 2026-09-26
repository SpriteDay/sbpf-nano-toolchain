use std::process::{ExitCode, Termination};

pub use nano_rustc_span::fatal_error::catch_fatal_errors;

/// Variant of `catch_fatal_erros` for the `interface::Result` return type
/// that also computes the exit code
pub fn catch_with_exit_code<T: Termination>(f: impl FnOnce() -> T) -> ExitCode {
    match catch_fatal_errors(f) {
        Ok(status) => status.report(),
        _ => ExitCode::FAILURE,
    }
}

pub fn main(left: u64, right: u64) -> ExitCode {
    let exit_code = catch_with_exit_code(|| run_compiler());

    exit_code
}
