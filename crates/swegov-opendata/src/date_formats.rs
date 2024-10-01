pub mod swe_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn to_string(date: &NaiveDateTime) -> String {
        date.format(FORMAT).to_string()
    }

    pub fn parse_from_str(s: &str) -> chrono::ParseResult<NaiveDateTime> {
        NaiveDateTime::parse_from_str(s, FORMAT)
    }

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&to_string(date))
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod option_swe_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    use super::swe_date_format;
    pub fn serialize<S>(opt_date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_date {
            None => serializer.serialize_none(),
            Some(date) => serializer.serialize_str(&swe_date_format::to_string(date)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let option_s = Option::<String>::deserialize(deserializer)?;
        match option_s {
            None => Ok(None),
            Some(s) => {
                let date = swe_date_format::parse_from_str(&s).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
        }
    }
}
