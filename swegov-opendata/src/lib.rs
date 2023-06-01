pub mod date_formats;
pub mod try_parse;

use crate::try_parse::{deserialize_tryparse_from_string, TryParse};

use serde_aux::field_attributes::deserialize_number_from_string;
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentStatusPage {
    pub dokumentstatus: DokumentStatus,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DokumentStatus {
    pub dokument: Dokument,
    pub dokuppgift: DokUppgift,
    pub dokbilaga: Option<DokBilaga>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dokument {
    pub dok_id: String,
    hangar_id: String,
    rm: String,
    beteckning: String,
    typ: String,
    subtyp: String,
    doktyp: Option<String>,
    typrubrik: Option<String>,
    dokumentnamn: String,
    debattnamn: Option<String>,
    tempbeteckning: String,
    organ: String,
    mottagare: Option<String>,
    #[serde(deserialize_with = "deserialize_tryparse_from_string")]
    nummer: TryParse<u64>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    slutnummer: u64,
    #[serde(with = "date_formats::swe_date_format")]
    datum: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format")]
    publicerad: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format")]
    systemdatum: NaiveDateTime,
    titel: String,
    subtitel: String,
    status: String,
    htmlformat: Option<String>,
    relaterat_id: Option<String>,
    source: Option<String>,
    sourceid: Option<String>,
    dokument_url_text: Option<String>,
    dokument_url_html: Option<String>,
    dokumentstatus_url_xml: Option<String>,
    utskottsforslag_url_xml: Option<String>,
    text: Option<String>,
    html: Option<String>,
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
    pub fn organ(&self) -> &str {
        &self.organ
    }
    pub fn html(&self) -> Option<&str> {
        self.html.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokUppgift {
    uppgift: Vec<Uppgift>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokBilaga {
    bilaga: Vec<Bilaga>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bilaga {
    dok_id: String,
    fil_url: String,
    filnamn: String,
    filstorlek: String,
    filtyp: String,
    subtitel: String,
    titel: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uppgift {
    dok_id: Option<String>,
    kod: String,
    namn: String,
    #[serde(with = "date_formats::option_swe_date_format", default)]
    systemdatum: Option<NaiveDateTime>,
    text: Option<String>,
}

use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentListaPage {
    dokumentlista: DokumentLista,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(deny_unknown_fields)]
pub struct DokumentLista {
    #[serde(rename = "@dDt")]
    d_dt: String,
    #[serde(rename = "@dPre")]
    d_pre: String,
    dokument: Vec<DokumentListaDokument>,
    #[serde(rename = "@nasta_sida")]
    nasta_sida: String,
    #[serde(rename = "@sida")]
    sida: String,
    #[serde(rename = "@q")]
    q: String,

    #[serde(rename = "@sidor")]
    sidor: String,
    #[serde(rename = "@traffar")]
    traffar: String,
    #[serde(rename = "@traff_fran")]
    traff_fran: String,
    #[serde(rename = "@traff_till")]
    traff_till: String,
    #[serde(rename = "@version")]
    version: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DokumentListaDokument {
    id: String,
    dok_id: String,
    traff: String,
    domain: String,
    database: String,
    datum: NaiveDate,
    #[serde(with = "date_formats::swe_date_format")]
    publicerad: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format")]
    systemdatum: NaiveDateTime,
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
