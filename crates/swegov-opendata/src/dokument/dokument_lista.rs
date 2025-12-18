use serde_with::serde_as;
use serde_with::DisplayFromStr;

use crate::date_formats::SweDate;
use crate::date_formats::SweDateTime;
use crate::dokument::shared;
use crate::shared::optionals;
use crate::try_parse::TryParse;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DokumentListaPage {
    dokumentlista: DokumentLista,
}

#[serde_as]
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields, rename = "dokumentlista")]
pub struct DokumentLista {
    #[serde(rename = "@dDt")]
    #[yaserde(rename = "dDt", attribute = true)]
    d_dt: String,
    #[serde(rename = "@dPre")]
    #[yaserde(rename = "dPre", attribute = true)]
    d_pre: String,
    #[serde(rename = "@dR")]
    #[yaserde(rename = "dR", attribute = true)]
    d_r: String,
    #[serde(rename = "@dSol")]
    #[yaserde(rename = "dSol", attribute = true)]
    d_sol: String,
    #[serde(rename = "@datum")]
    #[yaserde(attribute = true)]
    datum: SweDateTime,
    #[serde(rename = "@ms")]
    #[yaserde(attribute = true)]
    ms: String,
    #[serde(rename = "@nasta_sida")]
    #[yaserde(attribute = true)]
    pub nasta_sida: Option<String>,
    #[serde(rename = "@q")]
    #[yaserde(attribute = true)]
    pub q: String,
    #[serde(rename = "@sida")]
    #[serde_as(as = "DisplayFromStr")]
    #[yaserde(attribute = true)]
    sida: u64,
    #[serde(rename = "@sidor")]
    #[serde_as(as = "DisplayFromStr")]
    #[yaserde(attribute = true)]
    sidor: u64,
    #[serde(rename = "@traff_fran")]
    #[serde_as(as = "DisplayFromStr")]
    #[yaserde(attribute = true)]
    traff_fran: u64,
    #[serde(rename = "@traff_till")]
    #[serde_as(as = "DisplayFromStr")]
    #[yaserde(attribute = true)]
    traff_till: u64,
    #[serde(rename = "@traffar")]
    #[serde_as(as = "DisplayFromStr")]
    #[yaserde(attribute = true)]
    traffar: u64,
    #[serde(rename = "@varning")]
    #[yaserde(attribute = true)]
    varning: Option<String>,
    #[serde(rename = "@version")]
    #[yaserde(attribute = true)]
    version: String,
    facettlista: Option<String>,
    pub dokument: Vec<DokumentListaDokument>,
}

#[serde_as]
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct DokumentListaDokument {
    #[serde_as(as = "DisplayFromStr")]
    traff: u64,
    domain: String,
    database: String,
    datum: SweDate,
    id: String,
    rdrest: Option<String>,
    slutdatum: Option<String>,
    rddata: Option<String>,
    plats: Option<String>,
    klockslag: Option<String>,
    // // #[serde(with = "date_formats::option_swe_date_format")]
    // TODO this field can contain date (2018-03-07) and datetime (2016-02-11 15:28:15)
    publicerad: String,
    // #[serde(with = "date_formats::swe_date_format")]
    // systemdatum: NaiveDateTime,
    systemdatum: SweDateTime,
    undertitel: Option<String>,
    kalla: Option<String>,
    kall_id: Option<String>,
    pub dok_id: String,
    dokumentformat: Option<String>,
    dokument_url_html: String,
    dokument_url_text: String,
    inlamnad: Option<String>,
    motionstid: Option<String>,
    tilldelat: Option<String>,
    lang: Option<String>,
    url: Option<String>,
    relurl: Option<String>,
    titel: String,
    rm: String,
    organ: String,
    relaterat_id: Option<String>,
    doktyp: String,
    typ: String,
    subtyp: String,
    beteckning: Option<String>,
    tempbeteckning: Option<String>,
    nummer: TryParse<u64>,
    status: Option<String>,
    score: String,
    sokdata: SokData,
    summary: String,
    notisrubrik: String,
    notis: String,
    dokintressent: Option<String>,
    #[serde(deserialize_with = "optionals::deserialize_null_default")]
    filbilaga: FilBilaga,
    avdelning: String,
    struktur: Option<String>,
    audio: Option<String>,
    video: Option<String>,
    debattgrupp: Option<String>,
    debattdag: Option<String>,
    beslutsdag: Option<String>,
    beredningsdag: Option<String>,
    justeringsdag: Option<String>,
    beslutad: Option<String>,
    debattsekunder: Option<String>,
    ardometyp: Option<String>,
    reservationer: Option<String>,
    debatt: Option<String>,
    debattnamn: String,

    dokumentnamn: String,
    avdelningar: Option<shared::Avdelningar>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
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
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct Fil {
    typ: String,
    namn: String,
    storlek: u64,
    url: String,
}
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, yaserde::YaDeserialize, yaserde::YaSerialize,
)]
#[serde(deny_unknown_fields)]
pub struct SokData {
    titel: String,
    undertitel: String,
    soktyp: Option<String>,
    statusrad: String,
    // statusrad: NaiveDate,
    brodsmula: Option<String>,
    parti_kod: Option<String>,
    parti_namn: Option<String>,
    parti_website_url: Option<String>,
    parti_website_namn: Option<String>,
    parti_epost: Option<String>,
    parti_telefon: Option<String>,
    parti_telefontider: Option<String>,
    parti_logotyp_img_id: Option<String>,
    parti_logotyp_img_url: Option<String>,
    parti_logotyp_img_alt: Option<String>,
    parti_mandat: Option<String>,
    kalenderprio: Option<String>,
}
