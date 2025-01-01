use std::fmt::Display;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct SomeStruct {
    #[serde(serialize_with = "serialize_as_display")]
    field: usize,
}

// renaming this function won't rename the usage in `serialize_with`
pub fn serialize_as_display<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}
