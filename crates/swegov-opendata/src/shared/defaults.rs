use std::{fmt::Display, str::FromStr};

pub fn deserialize_default_from_empty_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Default + serde::Deserialize<'de>,
    T::Err: Display,
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(T::default())
    } else {
        T::from_str(&s).map_err(serde::de::Error::custom)
    }
    // Ok(opt.unwrap_or_default())
}
