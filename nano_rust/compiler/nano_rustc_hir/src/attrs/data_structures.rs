/// Crate type, as specified by `#![crate_type`
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub enum CrateType {
    /// `#![crate_type = "bin"]`
    Executable,
    /// `#![crate_type = "dylib"]`
    Dylib,
    /// `#![crate_type = "rlib"]` or `#![crate_type = "lib"]`
    #[default]
    Rlib,
    /// `#![crate_type = "staticlib"]`
    StaticLib,
    /// `#![crate_type = "cdylib"]`
    Cdylib,
    /// `#![crate_type = "proc-macro"]`
    ProcMacro,
}
