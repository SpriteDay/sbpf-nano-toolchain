use crate::config::*;

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum OptLevel {
    /// `-Copt-level=0`
    No,
    /// `-Copt-level=1`
    Less,
    /// `-Copt-level=2`
    More,
    /// `-Copt-level=3` / `-O`
    Aggressive,
    /// `-Copt-level=s`
    Size,
    /// `-Copt-level=z`
    SizeMin,
}

/// The top-level command-line options struct
pub struct Options {
    /// The crate config requested for the session, which may be combined
    /// with additional crate configurations during the compile process
    crate_types: Vec<CrateType>,
    optimize: OptLevel,
}
