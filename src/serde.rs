use std::fmt;

use serde::{
    de,
    ser::{self, Error},
};

pub(crate) fn serialize_as_ref<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
    T: AsRef<str>,
{
    let v = value.as_ref();
    serializer.serialize_str(v)
}

pub(crate) fn serialize_optional_as_ref<S, T>(
    value: &Option<T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
    T: AsRef<str>,
{
    if let Some(v) = value.as_ref() {
        serializer.serialize_some(v.as_ref())
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn serialize_json<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
    T: ser::Serialize,
{
    let json = serde_json::to_string(value).map_err(S::Error::custom)?;
    serializer.serialize_str(&json)
}

pub(crate) fn deserialize_string_as_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct StringVisitor;

    impl<'de> de::Visitor<'de> for StringVisitor {
        type Value = bool;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a string representing a bool value")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(match v.to_lowercase().chars().nth(0) {
                Some('t') | Some('1') => true,
                _ => false,
            })
        }
    }

    deserializer.deserialize_str(StringVisitor)
}
