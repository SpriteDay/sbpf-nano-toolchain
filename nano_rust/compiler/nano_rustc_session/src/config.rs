use std::cmp;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum OptionKind {
    /// An option that takes a value, and cannot appear more than once (e.g. `--out-dir`)
    ///
    /// Corresponds to [`getopts::Options::optopt`]
    Opt,

    /// An option that takes a value, and can appear multiple times (e.g. `--emit`)
    ///
    /// Corresponds to [`getopts::Options::optmulti`]
    Multi,

    /// An option that does not take a value, and cannot appear more than once (e.g. `--help`)
    ///
    /// Corresponds to [`getopts::Options::optflag`]
    /// The `hint` string must be empty
    Flag,

    /// An option that does not take a value, and can appear multiple times (e.g. `-O`)
    ///
    /// Corresponds to [`getopts::Options::optflagmulti`]
    FlagMulti,
}

pub struct NanoRustcOptGroup {
    /// The "primary" name for this option. Normally equal to `long_name`,
    /// except for options that don't have a long name, in which case
    /// `short_name` is used
    ///
    /// This is needed when interacting with `getopts` in some situations,
    /// because if an option has both forms, that library treats the long name
    /// as primary and the short name as an alias
    pub name: &'static str,
    kind: OptionKind,

    short_name: &'static str,
    long_name: &'static str,
    desc: &'static str,
    value_hint: &'static str,

    /// If true, this option should not be printed by `nano_rustc --help`, but
    /// should still be printed by `rustc --help -v`
    pub is_verbose_help_only: bool,
}

pub fn make_opt(
    kind: OptionKind,
    short_name: &'static str,
    long_name: &'static str,
    desc: &'static str,
    value_hint: &'static str,
) -> NanoRustcOptGroup {
    // "Flag" options don't have a value, and therefore don't have a value hint
    match kind {
        OptionKind::Opt | OptionKind::Multi => {}
        OptionKind::Flag | OptionKind::FlagMulti => assert_eq!(value_hint, ""),
    };
    NanoRustcOptGroup {
        name: cmp::max_by_key(short_name, long_name, |s| s.len()),
        kind,
        short_name,
        long_name,
        desc,
        value_hint,
        is_verbose_help_only: false,
    }
}

/// Returns all nano rustc command line options, including metadata for
/// each option, such as whether the option is stable.
///
/// # Option style guidelines
///
/// - `<param>`: Indicates a required parameter
/// - `[param]`: Indicates an optional parameter
/// - `|`: Indicates a mutually exclusive option
/// - `*`: a list element with description
pub fn nano_rustc_optgroups() -> Vec<NanoRustcOptGroup> {
    use OptionKind::{Flag, FlagMulti, Multi, Opt};

    use self::make_opt as opt;
}
