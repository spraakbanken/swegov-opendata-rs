use serde_with::serde_as;
use serde_with::{formats::PreferMany, OneOrMany};

#[serde_as]
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct DokIntressent {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub intressent: Vec<Intressent>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct Intressent {
    pub roll: String,
    pub namn: String,
    pub partibet: Option<String>,
    pub intressent_id: String,
    pub ordning: String,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokIntressentRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub intressent: Vec<IntressentRef<'a>>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct IntressentRef<'a> {
    pub roll: &'a str,
    pub namn: &'a str,
    pub partibet: Option<&'a str>,
    pub intressent_id: &'a str,
    pub ordning: &'a str,
}
