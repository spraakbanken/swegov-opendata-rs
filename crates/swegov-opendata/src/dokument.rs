use chrono::{NaiveDate, NaiveDateTime};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

use crate::date_formats;
use crate::try_parse::TryParse;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentStatusPage {
    pub dokumentstatus: DokumentStatus,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename = "dokumentstatus")]
pub struct DokumentStatus {
    pub dokument: Dokument,
    pub dokuppgift: DokUppgift,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dokbilaga: Option<DokBilaga>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dokument {
    pub dok_id: String,
    pub hangar_id: String,
    pub rm: String,
    pub beteckning: String,
    pub typ: String,
    pub subtyp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doktyp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typrubrik: Option<String>,
    pub dokumentnamn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debattnamn: Option<String>,
    pub tempbeteckning: String,
    pub organ: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mottagare: Option<String>,
    // #[serde(deserialize_with = "deserialize_tryparse_from_string")]
    // nummer: TryParse<u64>,
    pub nummer: String,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    pub slutnummer: String,
    #[serde(with = "date_formats::swe_date_format")]
    pub datum: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format")]
    pub publicerad: NaiveDateTime,
    #[serde(with = "date_formats::swe_date_format")]
    pub systemdatum: NaiveDateTime,
    pub titel: String,
    pub subtitel: String,
    pub status: String,
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
    pub html: String,
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
    // pub fn html(&self) -> Option<&str> {
    //     self.html.as_ref().map(|s| s.as_str())
    // }
    pub fn html(&self) -> &str {
        self.html.as_str()
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokUppgift {
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokBilaga {
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
    pub subtitel: String,
    pub titel: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uppgift {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dok_id: Option<String>,
    pub kod: String,
    pub namn: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "date_formats::option_swe_date_format",
        default
    )]
    pub systemdatum: Option<NaiveDateTime>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentListaPage {
    dokumentlista: DokumentLista,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, rename = "dokumentlista")]
pub struct DokumentLista {
    #[serde(rename = "@dDt")]
    d_dt: String,
    #[serde(rename = "@dPre")]
    d_pre: String,
    #[serde(rename = "@dR")]
    d_r: String,
    #[serde(rename = "@dSol")]
    d_sol: String,
    #[serde(rename = "@datum", with = "date_formats::swe_date_format")]
    datum: NaiveDateTime,
    #[serde(rename = "@ms")]
    ms: String,
    #[serde(rename = "@nasta_sida")]
    pub nasta_sida: Option<String>,
    #[serde(rename = "@q")]
    pub q: String,
    #[serde(rename = "@sida")]
    #[serde_as(as = "DisplayFromStr")]
    sida: u64,
    #[serde(rename = "@sidor")]
    #[serde_as(as = "DisplayFromStr")]
    sidor: u64,
    #[serde(rename = "@traff_fran")]
    #[serde_as(as = "DisplayFromStr")]
    traff_fran: u64,
    #[serde(rename = "@traff_till")]
    #[serde_as(as = "DisplayFromStr")]
    traff_till: u64,
    #[serde(rename = "@traffar")]
    #[serde_as(as = "DisplayFromStr")]
    traffar: u64,
    #[serde(rename = "@varning")]
    varning: Option<String>,
    #[serde(rename = "@version")]
    version: String,
    facettlista: Option<String>,
    pub dokument: Vec<DokumentListaDokument>,
}

#[serde_as]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
// #[serde(deny_unknown_fields)]
pub struct DokumentListaDokument {
    #[serde_as(as = "DisplayFromStr")]
    traff: u64,
    domain: String,
    database: String,
    datum: NaiveDate,
    id: String,
    rdrest: Option<String>,
    slutdatum: String,
    rddata: Option<String>,
    plats: String,
    klockslag: String,
    // // #[serde(with = "date_formats::option_swe_date_format")]
    // TODO this field can contain date (2018-03-07) and datetime (2016-02-11 15:28:15)
    publicerad: String,
    #[serde(with = "date_formats::swe_date_format")]
    systemdatum: NaiveDateTime,
    undertitel: String,
    kalla: String,
    kall_id: String,
    pub dok_id: String,
    dokumentformat: String,
    dokument_url_html: String,
    dokument_url_text: String,
    inlamnad: String,
    motionstid: String,
    tilldelat: String,
    lang: String,
    url: String,
    relurl: String,
    titel: String,
    rm: String,
    organ: String,
    relaterat_id: String,
    doktyp: String,
    typ: String,
    subtyp: String,
    beteckning: String,
    tempbeteckning: String,
    nummer: TryParse<u64>,
    status: String,
    score: String,
    sokdata: SokData,
    summary: String,
    notisrubrik: String,
    notis: String,
    dokintressent: Option<String>,
    #[serde(deserialize_with = "deserialize_null_default")]
    filbilaga: FilBilaga,
    avdelning: String,
    struktur: String,
    audio: String,
    video: String,
    debattgrupp: String,
    debattdag: String,
    beslutsdag: String,
    beredningsdag: String,
    justeringsdag: String,
    beslutad: String,
    debattsekunder: String,
    ardometyp: String,
    reservationer: String,
    debatt: Option<String>,
    debattnamn: String,

    dokumentnamn: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct FilBilaga {
    #[serde(skip_serializing_if = "Option::is_none")]
    fil: Option<Fil>,
}

impl Default for FilBilaga {
    fn default() -> Self {
        Self { fil: None }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fil {
    typ: String,
    namn: String,
    storlek: u64,
    url: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SokData {
    titel: String,
    undertitel: String,
    soktyp: String,
    statusrad: String,
    // statusrad: NaiveDate,
    brodsmula: String,
    parti_kod: String,
    parti_namn: String,
    parti_website_url: String,
    parti_website_namn: String,
    parti_epost: String,
    parti_telefon: String,
    parti_telefontider: String,
    parti_logotyp_img_id: String,
    parti_logotyp_img_url: String,
    parti_logotyp_img_alt: String,
    parti_mandat: String,
    kalenderprio: String,
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

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
