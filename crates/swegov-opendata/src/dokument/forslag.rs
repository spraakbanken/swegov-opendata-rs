use std::borrow::Cow;

use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::serde_as;
use serde_with::{formats::PreferMany, OneOrMany};
use uuid::Uuid;

use crate::shared::optionals;

#[cfg(test)]
mod tests;

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokForslag {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub forslag: Vec<Forslag>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokUtskottsForslag {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub utskottsforslag: Vec<UtskottsForslag>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokMotForslag {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub motforslag: Vec<MotForslag>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokForslagRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub forslag: Vec<ForslagRef<'a>>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokUtskottsForslagRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub utskottsforslag: Vec<UtskottsForslagRef<'a>>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokMotForslagRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub motforslag: Vec<MotForslagRef<'a>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Forslag {
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub hangar_id: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub nummer: usize,
    pub beteckning: Option<String>,
    #[serde(deserialize_with = "optionals::deserialize_null_default")]
    pub lydelse: String,
    pub lydelse2: Option<String>,
    pub utskottet: Option<String>,
    pub kammaren: Option<String>,
    pub behandlas_i: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behandlas_i_punkt: Option<String>,
    pub kammarbeslutstyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intressent: Option<String>,
    pub avsnitt: Option<String>,
    pub grundforfattning: Option<String>,
    pub andringsforfattning: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UtskottsForslag {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub punkt: u64,
    pub rubrik: String,
    pub forslag: Option<String>,
    pub forslag_del2: Option<String>,
    pub beteckning: Option<String>,
    pub beslut: Option<String>,
    pub beslutstyp: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub motforslag_nummer: u64,
    pub motforslag_partier: Option<String>,
    pub votering_id: Option<Uuid>,
    pub votering_sammanfattning_html: Option<serde_json::Value>,
    pub votering_ledamot_url_xml: Option<String>,
    pub votering_url_xml: Option<String>,
    pub rm: String,
    pub bet: String,
    pub vinnare: Option<String>,
    pub voteringskrav: Option<String>,
    pub beslutsregelkvot: Option<String>,
    pub beslutsregelparagraf: Option<String>,
    pub punkttyp: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SammanfattningHtml {
    // #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub table: serde_json::Value,
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub br: Vec<Option<String>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SammanfattningHtmlTable {
    pub tr: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SammanfattningHtmlTableRow {
    pub tr: Vec<SammanfattningHtmlTableRow>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotForslag {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub nummer: u64,
    pub rubrik: Option<String>,
    pub forslag: Option<String>,
    pub partier: String,
    pub typ: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub utskottsforslag_punkt: u64,
    pub id: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct ForslagRef<'a> {
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub hangar_id: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub nummer: usize,
    pub beteckning: Option<&'a str>,
    #[serde(deserialize_with = "optionals::deserialize_null_default")]
    pub lydelse: Cow<'a, str>,
    pub lydelse2: Option<&'a str>,
    pub utskottet: Option<&'a str>,
    pub kammaren: Option<&'a str>,
    pub behandlas_i: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behandlas_i_punkt: Option<&'a str>,
    pub kammarbeslutstyp: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intressent: Option<&'a str>,
    pub avsnitt: Option<&'a str>,
    pub grundforfattning: Option<&'a str>,
    pub andringsforfattning: Option<&'a str>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct UtskottsForslagRef<'a> {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub punkt: u64,
    pub rubrik: &'a str,
    #[serde(borrow)]
    pub forslag: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub forslag_del2: Option<Cow<'a, str>>,
    pub beslut: Option<&'a str>,
    pub beslutstyp: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub motforslag_nummer: u64,
    #[serde(borrow)]
    pub motforslag_partier: Option<Cow<'a, str>>,
    pub votering_id: Option<Uuid>,
    pub votering_sammanfattning_html: Option<serde_json::Value>,
    pub votering_url_xml: Option<&'a str>,
    pub votering_ledamot_url_xml: Option<&'a str>,
    pub rm: &'a str,
    pub bet: &'a str,
    pub vinnare: Option<&'a str>,
    pub voteringskrav: Option<&'a str>,
    pub beslutsregelkvot: Option<&'a str>,
    pub beslutsregelparagraf: Option<&'a str>,
    pub punkttyp: Option<&'a str>,
    pub beteckning: Option<&'a str>,
}

// #[serde_as]
// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(deny_unknown_fields)]
// pub struct SammanfattningHtml {
//     #[serde_as(as = "OneOrMany<_, PreferMany>")]
//     pub table: Vec<SammanfattningHtmlTable>,
//     #[serde_as(as = "OneOrMany<_, PreferMany>")]
//     pub br: Vec<Option<&'a str>>,
// }

// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(deny_unknown_fields)]
// pub struct SammanfattningHtmlTable {
//     pub tr: Vec<serde_json::Value>,
// }

// #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(deny_unknown_fields)]
// pub struct SammanfattningHtmlTableRow {
//     pub tr: Vec<SammanfattningHtmlTableRow>,
// }

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct MotForslagRef<'a> {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub nummer: u64,
    pub rubrik: Option<&'a str>,
    pub forslag: Option<&'a str>,
    #[serde(borrow)]
    pub partier: Cow<'a, str>,
    pub typ: &'a str,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub utskottsforslag_punkt: u64,
    pub id: Option<&'a str>,
}
