use std::sync::Arc;

use nano_rustc_target::spec::Target;

use crate::{
    config::{self},
    search_paths::SearchPath,
};

/// Represents the data associated with a compilation
/// session for a single crate
pub struct Session {
    pub target: Target,
    pub host: Target,
    pub opts: config::Options,
    pub target_tlib_path: Arc<SearchPath>,
}
