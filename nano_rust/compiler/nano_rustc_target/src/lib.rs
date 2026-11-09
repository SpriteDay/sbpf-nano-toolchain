pub mod spec;

macro_rules! target_spec_enum {
    (
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
    };
}
