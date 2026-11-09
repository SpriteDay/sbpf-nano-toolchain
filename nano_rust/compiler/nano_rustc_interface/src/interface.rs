use std::path::PathBuf;

use nano_rustc_target::spec::Target;

use nano_rustc_session::config::{self, Input, OutFileName};

/// Represents a compiler session. Note that every `Compiler` contains a
/// `Session`, but `Compiler` also contains some things that cannot be in
/// `Session`, due to `Session` being in a crate that has many fewer
/// dependencies than this crate
///
/// Can be used to run `nano_rustc_interface` queries
/// Created by passing [`Config`] to [`run_compiler`]
pub struct Compiler {
    pub sess: Session,
}

/// Represents the data associated with a compilation
/// session for a single crate
pub struct Session {
    pub target: Target,
}

/// The compiler configuration
pub struct Config {
    /// Command line options
    pub opts: config::Options,

    pub input: Input,
    pub output_dir: Option<PathBuf>,
    pub output_file: Option<OutFileName>,
}

pub fn run_compiler<R: Send>(config: Config, f: impl FnOnce(&Compiler) -> R + Send) -> R {}
