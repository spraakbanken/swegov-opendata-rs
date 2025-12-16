use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde_with::serde_as;
use serde_with::DisplayFromStr;

use crate::date_formats;
use crate::shared::optionals;
use crate::try_parse::TryParse;

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
#[serde(deny_unknown_fields)]
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
    #[serde(deserialize_with = "optionals::deserialize_null_default")]
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
    avdelningar: Avdelningar,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct FilBilaga {
    #[serde(skip_serializing_if = "Option::is_none")]
    fil: Option<Fil>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Avdelningar {
    avdelning: Vec<String>,
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
