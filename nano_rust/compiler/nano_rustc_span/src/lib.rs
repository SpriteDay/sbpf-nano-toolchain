#![feature(negative_impls)]

use std::path::PathBuf;

pub mod fatal_error;

/// The inner workings of `RealFileName`
///
/// It contains the `name`, `working_directory` and `embeddable_name` components
#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct InnerRealFileName {
    /// The name
    name: PathBuf,
    /// The working directory associated with the embeddable name
    working_directory: PathBuf,
    /// The embeddable name
    embeddable_name: PathBuf,
}

/// A self-contained "real" filename
#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct RealFileName {
    /// The local name (always present in the original crate)
    local: Option<InnerRealFileName>,
}

/// Differentiates between real files and common virtual files
#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum FileName {
    Real(RealFileName),
}
