use std::borrow::Cow;

use serde::{Deserialize, Deserializer};

pub fn string_or_seq_or_none_to_opt_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec;

    impl<'de> serde::de::Visitor<'de> for StringOrVec {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value.to_owned()))
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let vec: Vec<String> =
                Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(visitor))?;
            let mut curr_value: Option<String> = None;
            for value in vec.into_iter() {
                if value.is_empty() {
                    continue;
                }
                if let Some(curr) = &curr_value {
                    if curr != &value {
                        return Err(serde::de::Error::custom(format!("The sequence contains different non-empty values: (earlier)'{}' != '{}'(current) ", curr, value)));
                    }
                } else {
                    curr_value = Some(value);
                }
            }

            Ok(curr_value)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringOrVec)
}

pub fn string_or_seq_or_none_to_opt_cow_str<'de, D>(
    deserializer: D,
) -> Result<Option<Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec;

    impl<'de> serde::de::Visitor<'de> for StringOrVec {
        type Value = Option<Cow<'de, str>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(Cow::Owned(value.into())))
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let vec: Vec<String> =
                Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(visitor))?;
            let mut curr_value: Option<String> = None;
            for value in vec.into_iter() {
                if value.is_empty() {
                    continue;
                }
                if let Some(curr) = &curr_value {
                    if curr != &value {
                        return Err(serde::de::Error::custom(format!("The sequence contains different non-empty values: (earlier)'{}' != '{}'(current) ", curr, value)));
                    }
                } else {
                    curr_value = Some(value);
                }
            }
            match curr_value {
                Some(value) => Ok(Some(Cow::Owned(value))),
                None => Ok(None),
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringOrVec)
}
