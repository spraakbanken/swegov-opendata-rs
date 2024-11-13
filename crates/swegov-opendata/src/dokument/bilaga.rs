use std::borrow::Cow;

use serde_with::serde_as;
use serde_with::{formats::PreferMany, OneOrMany};

use crate::shared::optionals;

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokBilaga {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub bilaga: Vec<Bilaga>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bilaga {
    pub dok_id: String,
    pub fil_url: String,
    pub filnamn: String,
    pub filstorlek: String,
    pub filtyp: String,
    pub subtitel: Option<String>,
    #[serde(deserialize_with = "optionals::deserialize_null_default")]
    pub titel: String,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokBilagaRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub bilaga: Vec<BilagaRef<'a>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct BilagaRef<'a> {
    pub dok_id: &'a str,
    pub fil_url: &'a str,
    pub filnamn: &'a str,
    pub filstorlek: &'a str,
    pub filtyp: &'a str,
    pub subtitel: Option<Cow<'a, str>>,
    #[serde(deserialize_with = "optionals::deserialize_null_default", borrow)]
    pub titel: Cow<'a, str>,
}
