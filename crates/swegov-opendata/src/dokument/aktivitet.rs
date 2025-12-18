use chrono::NaiveDateTime;

use crate::date_formats::{self, SweDateTime};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct DokAktivitet {
    pub aktivitet: Vec<Aktivitet>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct Aktivitet {
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub hangar_id: Option<String>,
    pub kod: String,
    pub namn: String,
    // #[serde(with = "date_formats::swe_date_format")]
    pub datum: SweDateTime,
    pub status: Option<String>,
    pub ordning: String,
    pub process: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokAktivitetRef<'a> {
    pub aktivitet: Vec<AktivitetRef<'a>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct AktivitetRef<'a> {
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub hangar_id: Option<&'a str>,
    pub kod: &'a str,
    pub namn: &'a str,
    #[serde(with = "date_formats::swe_date_format")]
    pub datum: NaiveDateTime,
    pub status: Option<&'a str>,
    pub ordning: &'a str,
    pub process: Option<&'a str>,
}
