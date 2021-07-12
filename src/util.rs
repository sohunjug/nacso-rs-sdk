use serde::de;
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn split_deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(str_sequence
        .split(',')
        .map(|item| item.to_owned())
        .collect())
}
// fn split_deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let str_sequence: Option<String> = Option::deserialize(deserializer)?;
//     if let Some(s) = str_sequence {
//         return Ok(Some(s.split(',').map(|item| item.to_owned()).collect()));
//     }
//     Ok(None)
// }
#[allow(unused)]
pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn split_serialize<S>(clusters: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(ref d) = *clusters {
        return s.serialize_str(&d.join(","));
    }
    s.serialize_none()
}
