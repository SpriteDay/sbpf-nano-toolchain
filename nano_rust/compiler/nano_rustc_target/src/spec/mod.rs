use std::borrow::Cow;
use std::str::FromStr;

use nano_rustc_abi::Endian;

use crate::json::ToJson;

crate::target_spec_enum! {
    pub enum Arch {
        Sbf = "sbf",
    }
    other_variant = Other;
}

/// Everything `nano_rustc` knows about how to compile for a specific target
///
/// Every field here must be specified, and has no default value
#[derive(PartialEq, Clone, Debug)]
pub struct Target {
    /// Universioned target tuple to pass to LLVM
    ///
    /// Target tuples can optionally contain an OS version, which nano_rustc
    /// cannot know without querying the environment
    pub llvm_target: StaticCow<str>,
    /// Metadata about a target, for example the description or tier
    /// Used for generating target documentation
    pub metadata: TargetMetdata,
    /// Number of bits in a pointer. Influences the `taget_pointer_width` `cfg` variable
    pub pointer_width: u16,
    /// Architecture to use for ABI considerations
    pub arch: Arch,
    /// [Data Layout](https://llvm.org/docs/LangRef.html#data-layout) to pass to LLVM
    pub data_layout: StaticCow<str>,
    /// Optional settings with defaults
    pub options: TargetOptions,
}

/// Metadata about a target like the description or tier
#[derive(Default, PartialEq, Clone, Debug)]
pub struct TargetMetdata {
    /// A short description of the target including platform requirements,
    /// for example "64-bit Linux (kernel 3.2+, glibc 2.17+)"
    pub description: Option<StaticCow<str>>,
    /// The tier of the target. 1,2 or 3
    pub tier: Option<u64>,
    /// Whether the Rust project ships host tools for a target
    pub host_tools: Option<bool>,
    /// Whether a target has the `std` library. This is usually true for targets running
    /// on an operating system
    pub std: Option<bool>,
}

type StaticCow<T> = Cow<'static, T>;

/// Optional aspects of a target spec
///
/// This has an implementation of `Default`, see each field for what default is. In general,
/// these try to make "minimal defaults" that don't assume anything about the runtime they run in
///
/// `TargetOptions` as a separate structure is mostly an implementation detail of `Target`
/// construction, all its fields logically belong to `Target` and available from `Target`
/// through `Deref` impls
#[derive(PartialEq, Clone, Debug)]
pub struct TargetOptions {
    /// Used as the `target_endian` `cfg` variable. Defaults to little endian
    pub endian: Endian,
}
