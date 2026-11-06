use std::path::PathBuf;

use nano_rustc_session::config::{self, Input, OutFileName};

/// The compiler configuration
pub struct Config {
    /// Command line options
    pub opts: config::Options,

    pub input: Input,
    pub output_dir: Option<PathBuf>,
    pub output_file: Option<OutFileName>,
}
