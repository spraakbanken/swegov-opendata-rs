use std::borrow::Cow;

use chrono::NaiveDateTime;
use serde_aux::field_attributes::{
    deserialize_number_from_string, deserialize_option_number_from_string,
};
use serde_with::serde_as;
use serde_with::{formats::PreferMany, OneOrMany};

use crate::date_formats::{self, SweDateTime};
use crate::one_or_many::{
    string_or_seq_or_none_to_opt_cow_str, string_or_seq_or_none_to_opt_string,
};

#[serde_as]
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct Debatt {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub anforande: Vec<DebattAnforande>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct DebattAnforande {
    pub anf_beteckning: String,
    // #[serde(with = "date_formats::swe_date_format")]
    pub anf_datum: SweDateTime,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub anf_hangar_id: u64,
    pub anf_id: Option<String>,
    // pub anf_id: Uuid,
    pub anf_klockslag: String,
    pub anf_nummer: String,
    pub anf_rm: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub anf_sekunder: u64,
    pub anf_text: Option<String>,
    pub anf_typ: String,
    pub anf_video_id: String,
    // pub anf_video_id: Uuid,
    // #[serde(with = "date_formats::swe_date_format")]
    pub datumtid: SweDateTime,
    pub debatt_id: Option<String>,
    // pub debatt_id: Uuid,
    pub debatt_titel: Option<String>,
    pub debatt_typ: Option<String>,
    pub dok_beteckning: Option<String>,
    pub dok_id: Option<String>,
    pub dok_intressent: Option<String>,
    // #[serde(deserialize_with = "deserialize_default_from_empty_string")]
    pub intressent_id: Option<String>,
    pub kon: Option<String>,
    pub parent_id: Option<String>,
    // pub parent_id: Uuid,
    pub parti: String,
    #[serde(deserialize_with = "deserialize_option_number_from_string", default)]
    pub startpos: Option<u64>,
    #[serde(default)]
    pub systemdatum: Option<SweDateTime>,
    pub talare: String,
    pub talare_kort: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "string_or_seq_or_none_to_opt_string",
        default
    )]
    pub tumnagel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumnagel_stor: Option<String>,
    pub video_id: String,
    // pub video_id: Uuid,
    pub video_url: String,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub videostatus: Option<u64>,
    // pub videostatus: Option<String>,
    pub voteringspunkt: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DebattRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub anforande: Vec<DebattAnforandeRef<'a>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DebattAnforandeRef<'a> {
    pub anf_beteckning: &'a str,
    #[serde(with = "date_formats::swe_date_format")]
    pub anf_datum: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub anf_hangar_id: u64,
    pub anf_id: Option<&'a str>,
    // pub anf_id: Uuid,
    pub anf_klockslag: &'a str,
    pub anf_nummer: &'a str,
    pub anf_rm: &'a str,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub anf_sekunder: u64,
    #[serde(borrow)]
    pub anf_text: Option<Cow<'a, str>>,
    pub anf_typ: &'a str,
    pub anf_video_id: &'a str,
    // pub anf_video_id: Uuid,
    #[serde(with = "date_formats::swe_date_format")]
    pub datumtid: NaiveDateTime,
    pub debatt_id: Option<&'a str>,
    // pub debatt_id: Uuid,
    pub debatt_titel: Option<&'a str>,
    pub debatt_typ: Option<&'a str>,
    pub dok_beteckning: Option<&'a str>,
    pub dok_id: Option<&'a str>,
    pub dok_intressent: Option<&'a str>,
    // #[serde(deserialize_with = "deserialize_default_from_empty_string")]
    pub intressent_id: Option<&'a str>,
    pub kon: Option<&'a str>,
    pub parent_id: Option<&'a str>,
    // pub parent_id: Uuid,
    pub parti: &'a str,
    #[serde(deserialize_with = "deserialize_option_number_from_string", default)]
    pub startpos: Option<u64>,
    #[serde(with = "date_formats::option_swe_date_format", default)]
    pub systemdatum: Option<NaiveDateTime>,
    pub talare: &'a str,
    pub talare_kort: Option<&'a str>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "string_or_seq_or_none_to_opt_cow_str",
        default
    )]
    pub tumnagel: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumnagel_stor: Option<&'a str>,
    pub video_id: &'a str,
    // pub video_id: Uuid,
    pub video_url: &'a str,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub videostatus: Option<u64>,
    // pub videostatus: Option<&'a str>,
    pub voteringspunkt: Option<&'a str>,
}
