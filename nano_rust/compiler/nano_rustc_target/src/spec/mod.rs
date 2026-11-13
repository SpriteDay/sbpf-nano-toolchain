use std::borrow::Cow;
use std::collections::BTreeMap;
use std::str::FromStr;

use nano_rustc_abi::Endian;

/// Linker is called through a C/C++ compiler
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Cc {
    Yes,
    No,
}

/// Linker is LLD
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Lld {
    Yes,
    No,
}

/// All linkers have some kinds of command line interfaces and nano_rustc needs to know which commands
/// to use with each of them. So we cluster all such interfaces into a number of classes
/// that we call "linker flavors"
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LinkerFlavor {
    /// Unix-like linker with GNU extensions (both naked and compiler-wrapped forms).
    Gnu(Cc, Lld),
}

pub type LinkArgs = BTreeMap<LinkerFlavor, Vec<StaticCow<str>>>;

crate::target_spec_enum! {
    pub enum RelocModel {
        Static = "static",
        Pic = "pic",
        Pie = "pie",
        DynamicNoPic = "dynamic-no-pic",
        Ropi = "ropi",
        Rwpi = "rwpi",
        RopiRwpi = "ropi-rwpi",
    }

    parse_error_type = "relocation_model";
}

crate::target_spec_enum! {
    pub enum CodeModel {
        Tiny = "tiny",
        Small = "small",
        Kernel = "kernel",
        Medium = "medium",
        Large = "large",
    }

    parse_error_type = "code model";
}

crate::target_spec_enum! {
    pub enum BinaryFormat {
        Elf = "elf",
    }

    parse_error_type = "binary format";
}

crate::target_spec_enum! {
    pub enum RelroLevel {
        Full = "full",
        Partial = "partial",
        Off = "off",
        None = "none",
    }

    parse_error_type = "relro level";
}

crate::target_spec_enum! {
    pub enum Arch {
        Sbf = "sbf",
    }
    other_variant = Other;
}

crate::target_spec_enum! {
    pub enum Os {
        Solana = "solana",
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
    /// Width of c_int type
    pub c_int_width: u16,
    /// OS name to use for conditional compilation (`target_os`)
    pub os: Os,

    /// Linker to invoke
    pub linker: Option<StaticCow<str>>,
    /// Default linker flavor used if `-C linker-flavor` or `-C linker` are not passed
    /// on the command line. Defaults to `LinkerFlavor::Gnu(Cc::Yes, Lld::No)`
    pub linker_flavor: LinkerFlavor,

    /// Linker arguments that are passed *before* any user-defined libraries
    pub pre_link_args: LinkArgs,

    /// Optional link script applied to `dylib` and `executable` crate types.
    /// This is a string containing the script, not a path. Can only be applied
    /// to linkers where linker flavor matches `LinkerFlavor::Gnu(...)`
    pub link_script: Option<StaticCow<str>>,

    /// Default CPU to pass to LLVM. Corresponds to `llc -mcpu=$cpu`
    pub cpu: StaticCow<str>,
    /// Default (NanoRust) target features to enable for this target. These features
    /// overwrite `-Ctarget-cpu` but can be overwritten with `-Ctarget-features`.
    /// Corresponds to `llc -mattr=$llvm_features` where `$llvm_features` is the
    /// result of mapping the NanoRust features in this field to LLVM features
    pub features: StaticCow<str>,
    /// Whether dynamic linking is available on this target
    pub dynamic_linking: bool,
    /// If dynamic linking is available, whether only cdylibs are supported
    pub only_cdylib: bool,
    /// Relocation model, to use in object file. Corresponds to `llc
    /// -relocation-model=$relocation_model`
    pub relocation_model: RelocModel,
    /// Code model to use. Corresponds to `llc -code-model=$code_model`
    pub code_model: Option<CodeModel>,
    /// String to prepend to the name of every dynamic library
    pub dll_prefix: StaticCow<str>,
    /// String to append to the name of every dynamic library
    pub dll_suffix: StaticCow<str>,
    /// Target's binary file format
    pub binary_format: BinaryFormat,
    /// Whether to disable linking to the default libraries, typically corresponds
    /// to `-nodefaultlibs`
    pub no_default_libraries: bool,
    /// Dynamically linked executables can be compiled as position independent
    /// if the default relocation model of position independent code is not
    /// changed
    pub position_independent_executables: bool,
    /// Either partial, full, or off. Full RELRO makes the dynamic linker
    /// resolve all symbols at startup and marks the GOT read-only before
    /// starting the program, preventing overwriting the GOT
    pub relro_level: RelroLevel,
}
