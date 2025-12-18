use std::borrow::Cow;

use chrono::NaiveDateTime;
use serde_with::serde_as;
use serde_with::{formats::PreferMany, OneOrMany};

use crate::date_formats::{self, SweDateTime};

#[serde_as]
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct DokUppgift {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub uppgift: Vec<Uppgift>,
}

impl DokUppgift {
    pub fn get_by_kod(&self, kod: &str) -> Option<&String> {
        for uppgift in &self.uppgift {
            if uppgift.kod.as_str() == kod {
                return uppgift.text.as_ref();
            }
        }
        None
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct Uppgift {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dok_id: Option<String>,
    pub kod: String,
    pub namn: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        // with = "date_formats::option_swe_date_format",
        default
    )]
    pub systemdatum: Option<SweDateTime>,
    pub text: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokUppgiftRef<'a> {
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    pub uppgift: Vec<UppgiftRef<'a>>,
}

impl<'a> DokUppgiftRef<'a> {
    pub fn get_by_kod(&self, kod: &str) -> Option<&str> {
        for uppgift in &self.uppgift {
            if uppgift.kod == kod {
                return uppgift.text.as_deref();
            }
        }
        None
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct UppgiftRef<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dok_id: Option<&'a str>,
    pub kod: &'a str,
    pub namn: &'a str,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "date_formats::option_swe_date_format",
        default
    )]
    pub systemdatum: Option<NaiveDateTime>,
    #[serde(borrow)]
    pub text: Option<Cow<'a, str>>,
}
