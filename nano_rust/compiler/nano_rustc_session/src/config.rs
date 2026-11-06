use std::{cmp, collections::BTreeMap, path::PathBuf};

use nano_rustc_span::{FileName, edition::Edition};

pub use crate::options::*;

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

impl NanoRustcOptGroup {
    pub fn apply(&self, options: &mut getopts::Options) {
        let &Self {
            short_name,
            long_name,
            desc,
            value_hint,
            ..
        } = self;
        match self.kind {
            OptionKind::Opt => options.optopt(short_name, long_name, desc, value_hint),
            OptionKind::Multi => options.optmulti(short_name, long_name, desc, value_hint),
            OptionKind::Flag => options.optflag(short_name, long_name, desc),
            OptionKind::FlagMulti => options.optflagmulti(short_name, long_name, desc),
        };
    }
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

    let mut options = vec![
        opt(Multi, "", "emit", "", "<TYPE>[=<FILE>]"),
        opt(FlagMulti, "g", "", "Equivalent to -C debuginfo=2", ""),
        opt(Opt, "o", "", "Write output to FILENAME", "<FILENAME>"),
        opt(
            Opt,
            "",
            "out-dir",
            "Write output to compiler-chosen filename in DIR",
            "<DIR>",
        ),
        opt(
            Multi,
            "C",
            "codegen",
            "Set a codegen option",
            "<OPT>[=<VALUE>]",
        ),
        opt(Flag, "v", "verbose", "Use verbose output", ""),
    ];

    // Options in this list are hidden from `nano_rustc --help` by default, but are
    // shown by `rustc --help -v`
    let verbose_only: [NanoRustcOptGroup; 0] = [];
    options.extend(verbose_only.into_iter().map(|mut opt| {
        opt.is_verbose_help_only = true;
        opt
    }));

    options
}

#[derive(Clone)]
pub enum Input {
    /// Load source code from a file
    File(PathBuf),
    /// Load source code from a string
    Str {
        /// A string that is shown in place of a filename
        name: FileName,
    },
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum OutFileName {
    Real(PathBuf),
    Stdout,
}

pub use nano_rustc_hir::attrs::CrateType;

#[derive(Clone, Debug, Hash)]
pub struct OutputTypes(BTreeMap<OutputType, Option<OutFileName>>);

macro_rules! define_output_types {
    (
        $(
            $(#[doc = $doc:expr])*
            $Variant:ident => {
                shorthand: $shorthand:expr,
                extension: $extension:expr,
                description: $description:expr,
                default_filename: $default_filename:expr,
                is_text: $is_text:expr,
                compatible_with_cgus_and_single_output: $compatible:expr
            }
        ),* $(,)?
    ) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
        pub enum OutputType {
            $(
                $(#[doc = $doc])*
                $Variant,
            )*
        }

        impl OutputType {
            fn from_shorthand(shorthand: &str) -> Option<Self> {
                match shorthand {
                    $(
                        s if s == $shorthand => Some(OutputType::$Variant),
                    )*
                    _ => None,
                }
            }
        }
    };
}

define_output_types! {
    Assembly => {
        shorthand: "asm",
        extension: "s",
        description: "Generates a file with crate's assembly code",
        default_filename: "CRATE_NAME.s",
        is_text: true,
        compatible_with_cgus_and_single_output: false
    },
    Exe => {
        shorthand: "link",
        extension: "",
        description: "Generates the crates specified by --crate-type. This is the default if --emit is not specified",
        default_filename: "(platform and crate-type dependant)",
        is_text: false,
        compatible_with_cgus_and_single_output: false
    },
    LlvmAssembly => {
        shorthand: "llvm-ir",
        extension: "ll",
        description: "Generates a file containing LLVM IR",
        default_filename: "CRATE_NAME.ll",
        is_text: true,
        compatible_with_cgus_and_single_output: false
    },
}

fn parse_output_types(matches: &getopts::Matches) -> OutputTypes {
    let mut output_types = BTreeMap::new();
    if true {
        for list in matches.opt_strs("emit") {
            for output_type in list.split(',') {
                let (shorthand, path) = split_out_file_name(output_type);
                let output_type = OutputType::from_shorthand(shorthand)
                    .unwrap_or_else(|| panic!("Unknown emission type: `{shorthand}`"));
                output_types.insert(output_type, path);
            }
        }
    }
    if output_types.is_empty() {
        output_types.insert(OutputType::Exe, None);
    };
    OutputTypes(output_types)
}

fn split_out_file_name(arg: &str) -> (&str, Option<OutFileName>) {
    match arg.split_once('=') {
        None => (arg, None),
        Some((kind, "-")) => (kind, Some(OutFileName::Stdout)),
        Some((kind, path)) => (kind, Some(OutFileName::Real(PathBuf::from(path)))),
    }
}

fn parse_opt_level(matches: &getopts::Matches, cg: &CodegenOptions) -> OptLevel {
    // The `-O` and `-C opt-level` flags specify the same setting, so we want to be able
    // to use them interchangably. However, because they're technically different flags,
    // we need to work out manually which should take precedence if both are supplied (i.e.
    // the rightmost flag). We do this by finding the (rightmost) position of both flags and
    // comparing them. Note that if a flag is not found, its position will be `None`, which
    // always compared less than `Some(_)`
    let max_o = matches.opt_positions("O").into_iter().max();
    let max_c = matches
        .opt_strs_pos("C")
        .into_iter()
        .flat_map(|(i, s)| {
            // NB: This can match a string without `=`
            if let Some("opt-level") = s.split('=').next() {
                Some(i)
            } else {
                None
            }
        })
        .max();
    if max_o > max_c {
        OptLevel::Aggressive
    } else {
        match cg.opt_level.as_ref() {
            "0" => OptLevel::No,
            "1" => OptLevel::Less,
            "2" => OptLevel::More,
            "3" => OptLevel::Aggressive,
            "s" => OptLevel::Size,
            "z" => OptLevel::SizeMin,
            arg => {
                panic!(
                    "Optimization level needs to be \
                            between 0-3, s or z (instead was `{arg}`)"
                );
            }
        }
    }
}

pub fn build_session_options(matches: &getopts::Matches) -> Options {
    let edition = Edition::Edition2024;

    let unparsed_crate_types = matches.opt_strs("crate-type");
    let crate_types =
        parse_crate_types_from_list(unparsed_crate_types).unwrap_or_else(|e| panic!("{e}"));

    let output_types = parse_output_types(matches);

    let mut cg = CodegenOptions::build(matches);

    let cg = cg;

    let opt_level = parse_opt_level(matches, &cg);

    Options {
        crate_types,
        optimize: opt_level,
        output_types,
        edition,
    }
}

pub fn parse_crate_types_from_list(list_list: Vec<String>) -> Result<Vec<CrateType>, String> {
    let mut crate_types: Vec<CrateType> = Vec::new();
    for unparsed_crate_type in &list_list {
        for part in unparsed_crate_type.split(',') {
            let new_part = match part {
                "lib" => CrateType::default(),
                "rlib" => CrateType::Rlib,
                "staticlib" => CrateType::StaticLib,
                "dylib" => CrateType::Dylib,
                "cdylib" => CrateType::Cdylib,
                "bin" => CrateType::Executable,
                "proc-macro" => CrateType::ProcMacro,
                _ => {
                    return Err(format!(
                        "Unknown crate type: `{part}`, expected one of \
                    `lib`, `rlib`, `staticlib`, `dylib`, `cdylib`, `bin`, `proc-macro`"
                    ));
                }
            };
            if !crate_types.contains(&new_part) {
                crate_types.push(new_part)
            }
        }
    }

    Ok(crate_types)
}
