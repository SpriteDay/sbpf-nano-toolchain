pub mod json;
pub mod spec;

macro_rules! target_spec_enum {    (
        $( #[$attr:meta] )*
        pub enum $Name:ident {
            $(
                $( #[$variant_attr:meta] )*
                $Variant:ident = $string:literal $(,$alias:literal)* ,
            )*
        }
        parse_error_type = $parse_error_type:literal
    ) => {
        $( #[$attr] )*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
        #[derive(schemars::JsonSchema)]
        pub enum $Name {
            $(
                $( #[$variant_attr] )*
                #[serde(rename = $string)] // for JSON schema generation only
                $( #[serde(alias = $alias)] )*
                $Variant,
            )*
        }

        impl FromStr for $Name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $string => Self::$Variant,
                        $($alias => Self::$Variant,)*
                    )*
                    _ => {
                        let all = [$( concat!("'", $string, "'") ),*].join(", ");
                        return Err(format!("Invalid {}: '{s}'. Allowed values: {all}", $parse_error_type));
                    }
                })
            }
        }

        impl $Name {
            pub const ALL: &'static [$Name] = &[ $( $Name::$Variant, )* ];
            pub fn desc(&self) -> &'static str {
                match self {
                    $( Self::$Variant => $string, )*
                }
            }
        }
    };
    (
        $( #[$attr:meta] )*
        pub enum $Name:ident {
            $(
                $( #[$variant_attr:meta] )*
                $Variant:ident = $string:literal $(,$alias:literal)* ,
            )*
        }
        $( #[$other_variant_attr:meta] )*
        other_variant = $OtherVariant:ident;
    ) => {
        $( #[$attr] )*
        #[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
        pub enum $Name {
            $(
                $( #[$variant_attr:meta] )*
                 $( #[serde(alias = $alias)] )*
                $Variant,
            )*
            /// The vast majority of the time, the compiler deals with a fixed
            /// set of values, so it is convenient for them to be represetned in
            /// an enum. However, it is possible to have arbitrary values in a
            /// target JSON file (which can be parsed when `--target` is
            /// specified). This might occur, for example, for an out-of-tree
            /// codegen backend that supports a value (e.g. architecture or OS)
            /// that nano_rustc currently doesn't know about. This variant exists as
            /// an escape hatch for such cases
            $( #[$other_variant_attr] )*
            $OtherVariant(crate::spec::StaticCow<str>)
        }

        impl schemars::JsonSchema for $Name {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($Name))
            }

            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "string"
                })
            }
        }

        impl FromStr for $Name {
            type Err = std::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $string => Self::$Variant,
                        $($alias => Self::$Variant,)*
                    )*
                    _ => Self::$OtherVariant(s.to_owned().into()),
                })
            }
        }

        impl $Name {
            pub fn desc(&self) -> &str {
                match self {
                    $( Self::$Variant => $string, )*
                    Self::$OtherVariant(name) => name.as_ref(),
                }
            }
        }

        crate::target_spec_enum!(@common_impls $Name);
    };

    (@common_impls $Name:ident) => {
        impl crate::json::ToJson for $Name {
            fn to_json(&self) -> crate::json::Json {
                self.desc().to_json()
            }
        }

        crate::json::serde_deserialize_from_str!($Name);

        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.desc())
            }
        }
    }
}
use target_spec_enum;
