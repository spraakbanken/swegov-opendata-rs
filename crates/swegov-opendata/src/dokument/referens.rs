use serde_aux::field_attributes::deserialize_default_from_null;
use serde_with::serde_as;
use serde_with::{formats::PreferMany, OneOrMany};

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokReferens {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub referens: Vec<Referens>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Referens {
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hangar_id: Option<String>,
    pub referenstyp: Option<String>,
    pub uppgift: Option<String>,
    pub ref_dok_id: String,
    pub ref_dok_typ: String,
    pub ref_dok_rm: Option<String>,
    pub ref_dok_bet: Option<String>,
    #[serde(deserialize_with = "deserialize_default_from_null")]
    pub ref_dok_titel: String,
    pub ref_dok_subtitel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_dok_subtyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_dok_dokumentnamn: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokReferensRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub referens: Vec<ReferensRef<'a>>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct ReferensRef<'a> {
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hangar_id: Option<&'a str>,
    pub referenstyp: Option<&'a str>,
    pub uppgift: Option<&'a str>,
    pub ref_dok_id: &'a str,
    pub ref_dok_typ: &'a str,
    pub ref_dok_rm: Option<&'a str>,
    pub ref_dok_bet: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_default_from_null")]
    pub ref_dok_titel: &'a str,
    pub ref_dok_subtitel: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_dok_subtyp: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_dok_dokumentnamn: Option<&'a str>,
}
