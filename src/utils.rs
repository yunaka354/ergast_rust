use serde::Deserializer;
use serde::de::{self, Visitor};
use std::fmt;

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
