use nano_rustc_span::{RealFileName, edition::Edition};

use crate::config::*;

/// The top-level command-line options struct
pub struct Options {
    /// The crate config requested for the session, which may be combined
    /// with additional crate configurations during the compile process
    crate_types: Vec<CrateType>,
    optimize: OptLevel,
    output_types: OutputTypes,

    crate_name: Option<String>,
    edition: Edition,

    /// The (potentially remapped) working directory
    working_dir: RealFileName,
}
