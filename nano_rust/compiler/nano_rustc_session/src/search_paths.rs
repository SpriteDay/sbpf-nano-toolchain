use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone, Debug)]
pub struct SearchPath {
    pub kind: PathKind,
    pub dir: PathBuf,
    pub files: FilesIndex,
}

/// [FilesIndex] contains paths that can be efficiently looked up with (prefix, suffix) pairs
#[derive(Clone, Debug)]
pub struct FilesIndex(Vec<(Arc<str>, SearchPathFile)>);

#[derive(Clone, Debug)]
pub struct SearchPathFile {
    pub path: Arc<Path>,
    pub file_name_str: Arc<str>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub enum PathKind {
    Native,
    Crate,
    Dependency,
    Framework,
    All,
}
