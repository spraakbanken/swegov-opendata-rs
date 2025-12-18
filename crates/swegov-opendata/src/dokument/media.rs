use chrono::NaiveDateTime;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::serde_as;
use serde_with::{formats::PreferMany, OneOrMany};

use crate::date_formats::{self, SweDateTime};

#[serde_as]
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct WebbMedia {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub media: Vec<Media>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct Media {
    pub dok_id: String,
    pub url: String,
    pub videofileurl: String,
    pub audiofileurl: String,
    pub downloadurl: String,
    pub thumbnailurl: String,
    pub debateurl: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub debattsekunder: u64,
    pub debatt_rm: String,
    pub debatt_typ: String,
    pub dok_beteckning: String,
    // #[serde(with = "date_formats::swe_date_format")]
    pub datum: SweDateTime,
    // #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    // pub videostatus: Option<u64>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub videostatus: u64,
    pub source: String,
    pub inspelningstyp: String,
    // #[serde(with = "date_formats::swe_date_format")]
    pub systemdatum: SweDateTime,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct WebbMediaRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub media: Vec<MediaRef<'a>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct MediaRef<'a> {
    pub dok_id: &'a str,
    pub url: &'a str,
    pub videofileurl: &'a str,
    pub audiofileurl: &'a str,
    pub downloadurl: &'a str,
    pub thumbnailurl: &'a str,
    pub debateurl: &'a str,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub debattsekunder: u64,
    pub debatt_rm: &'a str,
    pub debatt_typ: &'a str,
    pub dok_beteckning: &'a str,
    #[serde(with = "date_formats::swe_date_format")]
    pub datum: NaiveDateTime,
    // #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    // pub videostatus: Option<u64>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub videostatus: u64,
    pub source: &'a str,
    pub inspelningstyp: &'a str,
    #[serde(with = "date_formats::swe_date_format")]
    pub systemdatum: NaiveDateTime,
}
