pub use serde_json::Value as Json;

pub trait ToJson {
    fn to_json(&self) -> Json;
}

impl ToJson for Json {
    fn to_json(&self) -> Json {
        self.clone()
    }
}

impl ToJson for str {
    fn to_json(&self) -> Json {
        Json::String(self.to_owned())
    }
}

macro_rules! serde_deserialize_from_str {
    ($ty:ty) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                FromStr::from_str(&s).map_err(serde::de::Error::custom)
            }
        }
    };
}
pub(crate) use serde_deserialize_from_str;
