use nano_rustc_span::{RealFileName, edition::Edition};

use crate::config::*;

/// The top-level command-line options struct
pub struct Options {
    /// The crate config requested for the session, which may be combined
    /// with additional crate configurations during the compile process
    pub crate_types: Vec<CrateType>,
    pub optimize: OptLevel,
    pub output_types: OutputTypes,

    pub edition: Edition,
}

pub mod parse {
    use std::str::FromStr;

    pub(crate) use super::*;

    /// Use this for any string option that has a static default
    pub(crate) fn parse_string(slot: &mut String, v: Option<&str>) -> bool {
        match v {
            Some(s) => {
                *slot = s.to_string();
                true
            }
            None => false,
        }
    }

    /// Use this for any string option that lacks a static default.
    pub(crate) fn parse_opt_string(slot: &mut Option<String>, v: Option<&str>) -> bool {
        match v {
            Some(s) => {
                *slot = Some(s.to_string());
                true
            }
            None => false,
        }
    }

    pub(crate) fn parse_list(slot: &mut Vec<String>, v: Option<&str>) -> bool {
        match v {
            Some(s) => {
                slot.extend(s.split_whitespace().map(|s| s.to_string()));
                true
            }
            None => false,
        }
    }

    pub(crate) fn parse_target_feature(slot: &mut String, v: Option<&str>) -> bool {
        match v {
            Some(s) => {
                if !slot.is_empty() {
                    slot.push(',');
                }
                slot.push_str(s);
                true
            }
            None => false,
        }
    }
}

/// Defines all `CodegenOptions`/`DebuggingOptions` fields and parsers all at once. The goal of this
/// macro is to define an interface that can be programmatically used by the option parser
/// to initialize the struct without hardcoding field names all over the place
macro_rules! options {
    ($struct_name:ident, $stat:ident, $optmod:ident, $prefix:expr, $outputname:expr,
    $($( #[$attr:meta] )* $opt:ident : $t:ty = (
        $init:expr,
        $parse:ident,
        $desc:expr
    )),* ,) => {
        #[derive(Clone)]
        pub struct $struct_name { $( $( #[$attr] )* pub $opt: $t ),* }

        impl Default for $struct_name {
            fn default() -> $struct_name {
                $struct_name { $($opt: $init),* }
            }
        }

        impl $struct_name {
            pub fn build(
                matches: &getopts::Matches,
            ) -> $struct_name {
                build_options(matches, $stat, $prefix, $outputname)
            }
        }

        pub const $stat: OptionDescrs<$struct_name> =
            &[ $( OptionDesc{ name: stringify!($opt), setter: $optmod::$opt,
                type_desc: desc::$parse, desc: $desc }),* ];

        mod $optmod {
        $(
            pub(super) fn $opt(cg: &mut super::$struct_name, v: Option<&str>) -> bool {
                super::parse::$parse(&mut redirect_field!(cg.$opt), v)
            }
        )*
        }
    };
}

// Sometimes different options need to build a common structure.
// That structure can be kept in one of the options' fields, the others become dummy
macro_rules! redirect_field {
    ($cg:ident.link_arg) => {
        $cg.link_args
    };
    ($cg:ident.pre_link_arg) => {
        $cg.pre_link_args
    };
    ($cg:ident.$field:ident) => {
        $cg.$field
    };
}

type OptionSetter<O> = fn(&mut O, v: Option<&str>) -> bool;
type OptionDescrs<O> = &'static [OptionDesc<O>];

pub struct OptionDesc<O> {
    name: &'static str,
    setter: OptionSetter<O>,
    // description for return value/type from mod desc
    type_desc: &'static str,
    // description for option from options table
    desc: &'static str,
}

fn build_options<O: Default>(
    matches: &getopts::Matches,
    descrs: OptionDescrs<O>,
    prefix: &str,
    outputname: &str,
) -> O {
    let mut op = O::default();
    for option in matches.opt_strs(prefix) {
        let (key, value) = match option.split_once('=') {
            None => (option, None),
            Some((k, v)) => (k.to_string(), Some(v)),
        };

        let option_to_lookup = key.replace('-', "_");
        match descrs
            .iter()
            .find(|opt_desc| opt_desc.name == option_to_lookup)
        {
            Some(OptionDesc {
                name: _,
                setter,
                type_desc,
                ..
            }) => {
                if !setter(&mut op, value) {
                    match value {
                        None => panic!(
                            "{outputname} option `{key}` requires {type_desc} (`-{prefix} {key}=<value>`)"
                        ),
                        Some(value) => panic!(
                            "Incorrect value `{value}` for {outputname} option `{key}` - {type_desc} was expected"
                        ),
                    }
                }
            }
            None => panic!("Unknown {outputname} option: `{key}`"),
        };
    }
    op
}

#[allow(non_upper_case_globals)]
mod desc {
    pub(crate) const parse_string: &str = "A string";
    pub(crate) const parse_opt_string: &str = parse_string;
    pub(crate) const parse_list: &str = "A space-separated list of strings";
    pub(crate) const parse_target_feature: &str = parse_string;
}

options! {
    CodegenOptions, CG_OPTIONS, cgopts, "C", "codegen",
    llvm_args: Vec<String> = (Vec::new(), parse_list, "A list of arguments to ass to LLVM (space separated)"),
    opt_level: String = ("0".to_string(), parse_string, "Optimization level (0-3, s, or z; default: 0)"),
    target_cpu: Option<String> = (None, parse_opt_string, "Select target processor"),
    target_feature: String = (String::new(), parse_target_feature, "Target specific attributes"),
}
