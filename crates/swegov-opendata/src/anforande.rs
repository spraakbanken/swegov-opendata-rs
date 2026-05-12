use std::borrow::Cow;

use crate::date_formats::SweDateTime;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnforandePage {
    pub anforande: Anforande,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct AnforandePageRef<'a> {
    pub anforande: AnforandeRef<'a>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename = "anforande", deny_unknown_fields)]
pub struct Anforande {
    pub dok_hangar_id: String,
    pub dok_id: String,
    pub dok_titel: String,
    pub dok_rm: String,
    pub dok_nummer: String,
    pub dok_datum: SweDateTime,
    pub avsnittsrubrik: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underrubrik: Option<String>,
    pub kammaraktivitet: Option<String>,
    pub anforande_id: String,
    pub anforande_nummer: String,
    pub talare: Option<String>,
    pub parti: String,
    pub anforandetext: Option<String>,
    pub intressent_id: Option<String>,
    pub rel_dok_id: Option<String>,
    pub replik: String,
    pub systemdatum: SweDateTime,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(
    rename = "anforande",
    deny_unknown_fields,
    bound(deserialize = "'de: 'a")
)]
pub struct AnforandeRef<'a> {
    pub dok_hangar_id: &'a str,
    pub dok_id: &'a str,
    pub dok_titel: &'a str,
    pub dok_rm: &'a str,
    pub dok_nummer: &'a str,
    pub dok_datum: SweDateTime,
    pub avsnittsrubrik: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underrubrik: Option<&'a str>,
    pub kammaraktivitet: Option<&'a str>,
    pub anforande_id: &'a str,
    pub anforande_nummer: &'a str,
    pub talare: Option<&'a str>,
    pub parti: Option<&'a str>,
    pub anforandetext: Option<Cow<'a, str>>,
    pub intressent_id: Option<&'a str>,
    pub rel_dok_id: Option<&'a str>,
    pub replik: &'a str,
    pub systemdatum: SweDateTime,
}
