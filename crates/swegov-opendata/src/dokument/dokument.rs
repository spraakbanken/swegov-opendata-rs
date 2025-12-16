use std::borrow::Cow;

use chrono::{NaiveDate, NaiveDateTime};

use crate::date_formats;
use crate::one_or_many;
use crate::shared::optionals;

pub use crate::dokument::aktivitet::{Aktivitet, DokAktivitet, DokAktivitetRef};
pub use crate::dokument::bilaga::{Bilaga, DokBilaga, DokBilagaRef};
pub use crate::dokument::debatt::Debatt;
pub use crate::dokument::forslag::{
    DokForslag, DokMotForslag, DokMotForslagRef, DokUtskottsForslag, DokUtskottsForslagRef, Forslag,
};
pub use crate::dokument::intressent::{DokIntressent, DokIntressentRef, Intressent, IntressentRef};
pub use crate::dokument::media::{WebbMedia, WebbMediaRef};
pub use crate::dokument::referens::{DokReferens, DokReferensRef, Referens};
pub use crate::dokument::uppgift::{DokUppgift, DokUppgiftRef, Uppgift};
pub use crate::dokument::{debatt::DebattRef, forslag::DokForslagRef};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentStatusPage {
    pub dokumentstatus: DokumentStatus,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokumentStatusPageRef<'a> {
    pub dokumentstatus: DokumentStatusRef<'a>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename = "dokumentstatus", deny_unknown_fields)]
pub struct DokumentStatus {
    pub dokument: Dokument,
    pub dokuppgift: Option<DokUppgift>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokbilaga: Option<DokBilaga>,
    pub dokintressent: Option<DokIntressent>,
    pub debatt: Option<Debatt>,
    pub dokaktivitet: Option<DokAktivitet>,
    pub dokforslag: Option<DokForslag>,
    pub dokreferens: Option<DokReferens>,
    pub webbmedia: Option<WebbMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokutskottsforslag: Option<DokUtskottsForslag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokmotforslag: Option<DokMotForslag>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(
    rename = "dokumentstatus",
    deny_unknown_fields,
    bound(deserialize = "'de: 'a")
)]
pub struct DokumentStatusRef<'a> {
    pub dokument: DokumentRef<'a>,
    pub dokuppgift: Option<DokUppgiftRef<'a>>,
    pub dokintressent: Option<DokIntressentRef<'a>>,
    pub debatt: Option<DebattRef<'a>>,
    pub dokaktivitet: Option<DokAktivitetRef<'a>>,
    pub dokforslag: Option<DokForslagRef<'a>>,
    pub dokreferens: Option<DokReferensRef<'a>>,
    pub webbmedia: Option<WebbMediaRef<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokbilaga: Option<DokBilagaRef<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokutskottsforslag: Option<DokUtskottsForslagRef<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokmotforslag: Option<DokMotForslagRef<'a>>,
}

// #[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dokument {
    pub dok_id: String,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub hangar_id: String,
    pub rm: String,
    pub beteckning: Option<String>,
    pub typ: String,
    pub subtyp: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "one_or_many::string_or_seq_or_none_to_opt_string"
    )]
    // #[serde_as(as = "OneOrMany<_, PreferOne>")]
    pub doktyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typrubrik: Option<String>,
    #[serde(default)]
    pub dokumentnamn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debattnamn: Option<String>,
    pub tempbeteckning: Option<String>,
    pub organ: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mottagare: Option<String>,
    // #[serde(deserialize_with = "deserialize_tryparse_from_string")]
    // nummer: TryParse<u64>,
    pub nummer: String,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub slutnummer: String,
    #[serde(with = "date_formats::swe_date_format")]
    pub datum: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format_or_empty_to_option")]
    pub publicerad: Option<NaiveDateTime>,
    #[serde(with = "date_formats::swe_date_format")]
    pub systemdatum: NaiveDateTime,
    #[serde(deserialize_with = "optionals::deserialize_null_default")]
    pub titel: String,
    pub subtitel: Option<String>,
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htmlformat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relaterat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokument_url_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokument_url_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokumentstatus_url_xml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utskottsforslag_url_xml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rubriker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl Dokument {
    pub fn dok_id(&self) -> &str {
        &self.dok_id
    }
    pub fn rm(&self) -> &str {
        &self.rm
    }
    pub fn datum(&self) -> NaiveDate {
        self.datum.date()
    }
    pub fn titel(&self) -> &str {
        &self.titel
    }
    // pub fn organ(&self) -> &str {
    //     &self.organ
    // }
    // pub fn html(&self) -> Option<&str> {
    //     self.html.as_ref().map(|s| s.as_str())
    // }
    pub fn html(&self) -> Option<&str> {
        self.html.as_deref()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, bound(deserialize = "'de: 'a"))]
pub struct DokumentRef<'a> {
    pub dok_id: &'a str,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub hangar_id: &'a str,
    pub rm: &'a str,
    pub beteckning: Option<&'a str>,
    pub typ: &'a str,
    pub subtyp: Option<&'a str>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "one_or_many::string_or_seq_or_none_to_opt_string"
    )]
    pub doktyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typrubrik: Option<&'a str>,
    #[serde(default)]
    pub dokumentnamn: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debattnamn: Option<&'a str>,
    pub tempbeteckning: Option<&'a str>,
    pub organ: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mottagare: Option<Cow<'a, str>>,
    // #[serde(deserialize_with = "deserialize_tryparse_from_string")]
    // nummer: TryParse<u64>,
    pub nummer: &'a str,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub slutnummer: &'a str,
    #[serde(with = "date_formats::swe_date_format")]
    pub datum: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format_or_empty_to_option")]
    pub publicerad: Option<NaiveDateTime>,
    #[serde(with = "date_formats::swe_date_format")]
    pub systemdatum: NaiveDateTime,
    #[serde(deserialize_with = "optionals::deserialize_null_default")]
    pub titel: String,
    pub subtitel: Option<Cow<'a, str>>,
    pub status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htmlformat: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relaterat_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokument_url_text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokument_url_html: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokumentstatus_url_xml: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utskottsforslag_url_xml: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretext: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rubriker: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none", borrow)]
    pub metadata: Option<Cow<'a, str>>,
}

impl<'a> DokumentRef<'a> {
    pub fn dok_id(&self) -> &str {
        &self.dok_id
    }
    pub fn rm(&self) -> &str {
        &self.rm
    }
    pub fn datum(&self) -> NaiveDate {
        self.datum.date()
    }
    pub fn titel(&self) -> &str {
        &self.titel
    }
    // pub fn organ(&self) -> &str {
    //     &self.organ
    // }
    // pub fn html(&self) -> Option<&str> {
    //     self.html.as_ref().map(|s| s.as_str())
    // }
    pub fn html(&self) -> Option<&str> {
        self.html.as_deref()
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
