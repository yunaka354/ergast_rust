use serde::de::{self, Visitor};
use serde::{self, Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

pub fn deserialize_string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringToI32Visitor;

    impl<'de> Visitor<'de> for StringToI32Visitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string that can be parsed into an i32")
        }

        fn visit_str<E>(self, value: &str) -> Result<i32, E>
        where
            E: de::Error,
        {
            value.parse::<i32>().map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_str(StringToI32Visitor)
}

pub fn deserialize_string_to_option_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref s) if s.is_empty() => Ok(None),
        Some(s) => i32::from_str(&s)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub fn deserialize_string_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    f32::from_str(&s).map_err(serde::de::Error::custom)
}
