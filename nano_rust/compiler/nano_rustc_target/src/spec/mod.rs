use std::borrow::Cow;

/// Everything `nano_rustc` knows about how to compile for a specific target
///
/// Every field here must be specified, and has no default value
pub struct Target {
    /// Universioned target tuple to pass to LLVM
    ///
    /// Target tuples can optionally contain an OS version, which nano_rustc
    /// cannot know without querying the environment
    pub llvm_target: StaticCow<str>,
}

type StaticCow<T> = Cow<'static, T>;
