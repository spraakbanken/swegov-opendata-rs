// copied from: https://gist.github.com/rust-play/e7320f84b125bcac0c41f349b03d2f46

use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum TryParse<T> {
    Parsed(T),
    Unparsed(Value),
    NotPresent,
}

impl<T: Clone> Clone for TryParse<T> {
    fn clone(&self) -> Self {
        match self {
            TryParse::NotPresent => TryParse::NotPresent,
            TryParse::Parsed(t) => TryParse::Parsed(t.clone()),
            TryParse::Unparsed(v) => TryParse::Unparsed(v.clone()),
        }
    }
}

// #[derive(Deserialize, Debug)]
// struct Foo4 {
//     inner: TryParse<Inner>,

//     #[serde(flatten)]
//     other: HashMap<String, Value>,
// }

// #[derive(Deserialize, Debug, Default)]
// struct Inner {
//     i1: i32,
//     i2: i32,
// }

impl<'de, T: DeserializeOwned> Deserialize<'de> for TryParse<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Option::<Value>::deserialize(deserializer)? {
            None => Ok(TryParse::NotPresent),
            Some(value) => match T::deserialize(&value) {
                Ok(t) => Ok(TryParse::Parsed(t)),
                Err(_) => Ok(TryParse::Unparsed(value)),
            },
        }
    }
}

// impl<'de, T: Deserialize<'de> + Clone> Deserialize<'de> for TryParse<T> {
//     fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
//         match Option::<Value>::deserialize(deserializer)? {
//             None => Ok(TryParse::NotPresent),
//             Some(value) => match T::deserialize(&value) {
//                 Ok(t) => Ok(TryParse::Parsed(t.clone())),
//                 Err(_) => Ok(TryParse::Unparsed(value.clone())),
//             },
//         }
//     }
// }

impl<T: Serialize> Serialize for TryParse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TryParse::NotPresent => serializer.serialize_none(),
            TryParse::Parsed(t) => t.serialize(serializer),
            TryParse::Unparsed(v) => v.serialize(serializer),
        }
    }
}

impl<T: FromStr> FromStr for TryParse<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match T::from_str(s) {
            Ok(t) => Ok(TryParse::Parsed(t)),
            Err(_) => Ok(TryParse::Unparsed(s.into())),
        }
    }
}

pub fn deserialize_tryparse_from_string<'de, T, D>(deserializer: D) -> Result<TryParse<T>, D::Error>
where
    D: Deserializer<'de>,
    // T: FromStr + serde::Deserialize<'de> + Clone,
    T: FromStr + DeserializeOwned,
    <T as FromStr>::Err: fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrTryParse<T> {
        String(String),
        Value(T),
    }

    match StringOrTryParse::<T>::deserialize(deserializer)? {
        StringOrTryParse::String(s) => s.parse::<TryParse<T>>().map_err(serde::de::Error::custom),
        StringOrTryParse::Value(t) => Ok(TryParse::Parsed(t)),
    }
}
